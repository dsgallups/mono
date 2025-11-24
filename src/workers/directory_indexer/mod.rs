use std::str::FromStr;

use embed_db::{EMBED_DB, NewEmbed};
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

mod processor;
use processor::*;
use tracing::info;

use crate::{
    models::{file_chunks, files, index_tasks},
    views::IndexStatus,
};

pub struct Worker {
    pub ctx: AppContext,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct WorkerArgs {
    pub task_id: i32,
}

#[async_trait]
impl BackgroundWorker<WorkerArgs> for Worker {
    /// Creates a new instance of the Worker with the given application context.
    ///
    /// This function is called when registering the worker with the queue system.
    ///
    /// # Parameters
    /// * `ctx` - The application context containing shared resources
    fn build(ctx: &AppContext) -> Self {
        Self { ctx: ctx.clone() }
    }

    /// Returns the class name of the worker.
    ///
    /// This name is used when enqueueing jobs and identifying the worker in logs.
    /// The implementation returns the struct name as a string.
    fn class_name() -> String {
        "DirectoryIndexer".to_string()
    }

    /// Returns tags associated with this worker.
    ///
    /// Tags can be used to filter which workers run during startup.
    /// The default implementation returns an empty vector (no tags).
    fn tags() -> Vec<String> {
        Vec::new()
    }

    /// Performs the actual work when a job is processed.
    ///
    /// This is the main function that contains the worker's logic.
    /// It gets executed when a job is dequeued from the job queue.
    ///
    /// # Returns
    /// * `Result<()>` - Ok if the job completed successfully, Err otherwise
    async fn perform(&self, args: WorkerArgs) -> Result<()> {
        info!("=================DirectoryIndexer=======================");

        let task = index_tasks::Entity::find_by_id(args.task_id)
            .one(&self.ctx.db)
            .await?
            .ok_or(Error::NotFound)?;

        tracing::info!("Performing task {task:?}");

        let (tx_event, mut rx_event) = mpsc::unbounded_channel::<IndexEvent>();

        //todo: FileIndexer could be shutdown gracefully.
        tokio::task::spawn(FileIndexer::new(task.path.clone()).run(tx_event));

        let mut entry_count = None;
        let mut entries_processed = 0;

        //todo: need to shut down gracefully
        while let Some(rx) = rx_event.recv().await {
            info!("ev {rx:?}");
            let new_registration = match rx {
                IndexEvent::AccessError(_io) => {
                    entries_processed += 1;
                    //you would do something like save this error, etc.
                    continue;
                }
                IndexEvent::FinishedWithNoop => {
                    entries_processed += 1;
                    continue;
                }
                IndexEvent::DirectoryWalked(count) => {
                    entry_count = Some(count);
                    continue;
                }
                IndexEvent::Read { path: _, err: _ } => {
                    entries_processed += 1;
                    //something else
                    continue;
                }
                IndexEvent::EmbeddingFailure(_path) => {
                    entries_processed += 1;
                    continue;
                }
                IndexEvent::Register(file) => file,
            };

            entries_processed += 1;
            let mut task_am = task.clone().into_active_model();
            if let Some(entry_count) = entry_count {
                task_am.progress = Set(entries_processed as f32 / entry_count as f32);
            }
            task_am.queue = Set(new_registration.path.to_string_lossy().into_owned());
            if let Ok(model) = task_am.update(&self.ctx.db).await {
                let Ok(status) = IndexStatus::from_str(&model.status);
                if matches!(status, IndexStatus::Cancelled) {
                    // we should cancel threads here ideally.
                    return Ok(());
                }
            }

            //yes, bad. I know. Hope you don't run this on windows
            let path = new_registration.path.to_string_lossy().into_owned();
            let title = new_registration
                .path
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or(path.clone());
            let file_type = new_registration.contents.file_type();

            let Ok(model) = files::ActiveModel {
                title: Set(title),
                path: Set(path),
                file_type: Set(file_type.to_string()),
                ..Default::default()
            }
            .insert(&self.ctx.db)
            .await
            else {
                continue;
            };
            let Some(embeddings) = new_registration.contents.chunks() else {
                continue;
            };

            let mut new_embeds = Vec::with_capacity(embeddings.len());

            for embed in embeddings.iter() {
                let file_chunk = file_chunks::ActiveModel {
                    content: Set(embed.text().to_string()),
                    file_id: Set(model.id),
                    ..Default::default()
                }
                .insert(&self.ctx.db)
                .await
                .unwrap();

                new_embeds.push(NewEmbed {
                    id: file_chunk.id as usize,
                    embeds: embed.embeddings(),
                });
            }
            EMBED_DB.insert(new_embeds);
        }

        let mut am = task.into_active_model();
        am.status = Set(IndexStatus::Cancelled.to_string());
        _ = am.save(&self.ctx.db).await;
        info!("FINISHED!");
        Ok(())
    }
}
