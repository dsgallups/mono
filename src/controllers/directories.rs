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
pub async fn list_directory_contents(Query(params): Query<Params>) -> Result<Json<Vec<String>>> {
    let path_buf = PathBuf::from(params.path);

    let mut contents = Vec::new();
    let Ok(mut directory_contents) = fs::read_dir(&path_buf).await.inspect_err(|e| {
        tracing::warn!("On checking directory contents: {e}");
    }) else {
        return Ok(Json(contents));
    };

    while let Ok(Some(next)) = directory_contents.next_entry().await {
        contents.push(next.file_name().to_string_lossy().to_string());
    }

    Ok(Json(contents))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/directories/")
        .add("/", get(list_directory_contents))
}
