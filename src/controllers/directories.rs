#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use std::path::PathBuf;

use loco_rs::prelude::*;
use serde::Deserialize;
use tokio::fs;

#[derive(Deserialize)]
pub struct Params {
    pub path: String,
}

#[debug_handler]
pub async fn perform_indexing_task(State(ctx): State<AppContext>) {
    //todo
}

#[debug_handler]
pub async fn index(
    State(ctx): State<AppContext>,
    Query(params): Query<Params>,
) -> Result<Response> {
    let path_buf = PathBuf::from(params.path);

    let directory_contents = fs::read_dir(&path_buf).await;

    format::empty()
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/directories/")
        .add("/", get(index))
}
