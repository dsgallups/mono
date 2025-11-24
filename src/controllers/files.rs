#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use embed_db::{EMBEDDER, EMBED_DB};
use loco_rs::prelude::*;
use sea_orm::PaginatorTrait;
use serde::Deserialize;

use crate::{
    models::{
        _entities::files::{Entity, Model},
        file_chunks, files,
    },
    views::{FileChunk, FileSimilarity, FileType},
};

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

#[derive(Deserialize, Debug)]
pub struct Params {
    q: Option<String>,
}

#[debug_handler]
pub async fn list(
    State(ctx): State<AppContext>,
    Query(params): Query<Params>,
) -> Result<Json<Vec<FileSimilarity>>> {
    let Some(prompt) = params.q else {
        let models = Entity::find().paginate(&ctx.db, 50).fetch_page(0).await?;
        tracing::info!("params: {params:?}");
        return Ok(Json(models.into_iter().map(Into::into).collect()));
    };

    let embedding: Vec<f32> = {
        let mut lock = EMBEDDER.lock().await;
        let result = lock.naive_embed(&prompt).unwrap().squeeze(0).unwrap();
        result.to_vec1().unwrap()
    };

    if embedding.is_empty() {
        return Ok(Json(vec![]));
    }

    let neighbors: HashMap<i32, f32> = EMBED_DB
        .get(&embedding)
        .into_iter()
        .map(|n| (n.id as i32, n.similarity))
        .collect();

    let db_chunks = file_chunks::Entity::find()
        .filter(file_chunks::Column::Id.is_in(neighbors.keys().copied()))
        .all(&ctx.db)
        .await
        .unwrap();
    let file_ids: HashSet<i32> = db_chunks.iter().map(|m| m.file_id).collect();

    struct FileData {
        title: String,
        file_type: FileType,
        path: String,
        chunks: Vec<FileChunk>,
    }

    // this is bad but like sea orm and I have a troubled past.
    let mut files: HashMap<i32, FileData> = files::Entity::find()
        .filter(files::Column::Id.is_in(file_ids))
        .all(&ctx.db)
        .await
        .unwrap()
        .into_iter()
        .map(|file| {
            let Ok(file_type) = FileType::from_str(&file.file_type);
            (
                file.id,
                FileData {
                    title: file.title,
                    path: file.path,
                    file_type,
                    chunks: Vec::new(),
                },
            )
        })
        .collect();

    //let mut results = Vec::with_capacity(files.len());

    for chunk in db_chunks {
        let files = files.get_mut(&chunk.file_id).unwrap();
        let similarity = neighbors.get(&chunk.id).unwrap();
        files.chunks.push(FileChunk {
            content: chunk.content,
            similarity: *similarity,
        });
    }

    let result = files
        .into_iter()
        .map(|(id, data)| FileSimilarity {
            id,
            title: data.title,
            file_type: data.file_type,
            path: data.path,
            chunks: data.chunks,
        })
        .collect();
    Ok(Json(result))
}

#[debug_handler]
pub async fn get_one(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    format::json(load_item(&ctx, id).await?)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/files/")
        .add("/", get(list))
        .add("{id}", get(get_one))
}
