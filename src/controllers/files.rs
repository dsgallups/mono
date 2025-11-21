#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    models::_entities::files::{ActiveModel, Entity, Model},
    views::FileResponse,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub title: String,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.title = Set(self.title.clone());
    }
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

#[debug_handler]
pub async fn list(State(_ctx): State<AppContext>) -> Result<Json<Vec<FileResponse>>> {
    // let models = Entity::find().all(&ctx.db).await?;

    // Ok(Json(models.into_iter().map(Into::into).collect()))

    let files = vec![
        FileResponse::new(1, "poly.txt"),
        FileResponse::new(2, "another file.txt"),
        FileResponse::new(2, "third_file.jpg"),
    ];

    Ok(Json(files))
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
