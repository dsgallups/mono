#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use loco_rs::prelude::*;
use serde::Deserialize;

use crate::{
    models::_entities::index_tasks::{ActiveModel, Entity, Model},
    views::IndexResponse,
    workers::directory_indexer,
};

#[derive(Deserialize, Debug)]
pub struct Params {
    pub path: String,
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

#[debug_handler]
pub async fn list(State(ctx): State<AppContext>) -> Result<Json<Vec<IndexResponse>>> {
    let index_tasks = Entity::find().all(&ctx.db).await?;

    Ok(Json(index_tasks.into_iter().map(Into::into).collect()))
}

#[debug_handler]
pub async fn add(State(ctx): State<AppContext>, Json(params): Json<Params>) -> Result<Response> {
    let item = ActiveModel {
        path: Set(params.path),
        queue: Set("Starting Task".to_string()),
        ..Default::default()
    };
    let item = item.insert(&ctx.db).await?;

    directory_indexer::Worker::perform_later(
        &ctx,
        directory_indexer::WorkerArgs { task_id: item.id },
    )
    .await?;
    format::json(item)
}

#[debug_handler]
pub async fn remove(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    load_item(&ctx, id).await?.delete(&ctx.db).await?;
    format::empty()
}

#[debug_handler]
pub async fn get_one(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    format::json(load_item(&ctx, id).await?)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/index_tasks/")
        .add("/", get(list))
        .add("/", post(add))
        .add("{id}", get(get_one))
        .add("{id}", delete(remove))
}
