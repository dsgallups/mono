use file_indexer::{FileIndexer, IndexEvent, IndexRequest};
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::{
    hnsw::{NewEmbed, EMBED_DB},
    models::{file_chunks, files, index_tasks},
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
        println!("=================DirectoryIndexer=======================");

        let task = index_tasks::Entity::find_by_id(args.task_id)
            .one(&self.ctx.db)
            .await?
            .ok_or(Error::NotFound)?;

        tracing::info!("Performing task {task:?}");

        let (tx_event, mut rx_event) = mpsc::unbounded_channel::<IndexEvent>();

        //todo: this channel will cleanup if a graceful shutdown is possible.
        let (_tx_req, rx_req) = mpsc::unbounded_channel::<IndexRequest>();

        tokio::task::spawn(FileIndexer::new(task.path, rx_req).run(tx_event));

        //todo: need to shut down gracefully
        while let Some(rx) = rx_event.recv().await {
            let new_registration = match rx {
                IndexEvent::AccessError(_io) => {
                    //you would do something like save this error, etc.
                    continue;
                }
                IndexEvent::DirectoryWalked => {
                    continue;
                }
                IndexEvent::Read { path: _, err: _ } => {
                    //something else
                    continue;
                }
                IndexEvent::EmbeddingFailure(_path) => {
                    continue;
                }
                IndexEvent::Register(file) => file,
            };

            //yes, bad. I know. Hope you don't run this on windows
            let path = new_registration.path.to_string_lossy().into_owned();
            let title = new_registration
                .path
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or(path.clone());

            let Ok(model) = files::ActiveModel {
                title: Set(title),
                path: Set(path),
                ..Default::default()
            }
            .insert(&self.ctx.db)
            .await
            else {
                continue;
            };
            let Some(embeddings) = new_registration.contents.embeddings() else {
                continue;
            };

            let mut new_embeds = Vec::with_capacity(embeddings.len());

            for embed in embeddings.iter() {
                let file_chunk = file_chunks::ActiveModel {
                    content: Set("".to_string()),
                    file_id: Set(model.id),
                    ..Default::default()
                }
                .insert(&self.ctx.db)
                .await
                .unwrap();
                new_embeds.push(NewEmbed {
                    id: file_chunk.id as usize,
                    embeds: embed,
                });
            }

            EMBED_DB.insert(new_embeds);
        }

        Ok(())
    }
}
