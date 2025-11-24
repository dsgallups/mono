#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use embed_db::{EMBED_DB, EMBEDDER};
use loco_rs::prelude::*;
use sea_orm::PaginatorTrait;
use serde::Deserialize;
use tokio::fs;

use crate::{
    models::{
        _entities::files::{Entity, Model},
        file_chunks, files,
    },
    views::{FileChunk, FileDetails, FileSimilarity, FileType},
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
            id: chunk.id,
            content: chunk.content,
            similarity: *similarity,
        });
    }

    let mut result: Vec<FileSimilarity> = files
        .into_iter()
        .map(|(id, mut data)| {
            data.chunks
                .sort_by(|a, b| b.similarity.total_cmp(&a.similarity));
            FileSimilarity {
                id,
                title: data.title,
                file_type: data.file_type,
                path: data.path,
                chunks: data.chunks,
            }
        })
        .collect();
    result.sort_by(|a, b| {
        let a_max = a.chunks.first().map(|v| v.similarity).unwrap_or_default();
        let b_max = b.chunks.first().map(|v| v.similarity).unwrap_or_default();

        b_max.total_cmp(&a_max)
    });

    Ok(Json(result))
}

#[derive(Deserialize, Debug)]
pub struct ChunkParams {
    chunk: Option<i32>,
}

#[debug_handler]
pub async fn get_one(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Query(params): Query<ChunkParams>,
) -> Result<Json<FileDetails>> {
    let file = load_item(&ctx, id).await?;

    let Ok(file_type) = FileType::from_str(&file.file_type);
    let mut response = FileDetails {
        id: file.id,
        content: String::new(),
        title: file.title,
        path: file.path,
        file_type,
        chunks: Vec::new(),
    };

    if let Ok(contents) = fs::read_to_string(&response.path).await {
        response.content = contents;
    }

    if let Some(id) = params.chunk
        && let Ok(Some(file_chunk)) = file_chunks::Entity::find()
            .filter(file_chunks::Column::Id.eq(id))
            .one(&ctx.db)
            .await
    {
        response.chunks.push(FileChunk {
            id: file_chunk.id,
            content: file_chunk.content,
            similarity: 1.,
        });
    }

    Ok(Json(response))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/files/")
        .add("/", get(list))
        .add("{id}", get(get_one))
}
