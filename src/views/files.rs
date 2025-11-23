use serde::Serialize;

use crate::models::files;

#[derive(Serialize)]
pub struct FileResponse {
    id: i32,
    title: String,
}
impl FileResponse {
    pub fn new(id: i32, title: impl Into<String>) -> Self {
        Self {
            id,
            title: title.into(),
        }
    }
}

impl From<files::Model> for FileResponse {
    fn from(value: files::Model) -> Self {
        Self {
            id: value.id,
            title: value.title,
        }
    }
}

#[derive(Serialize)]
pub struct FileChunk {
    pub content: String,
    pub similarity: f32,
}

#[derive(Serialize)]
pub struct FileSimilarity {
    pub id: i32,
    pub title: String,
    pub path: String,
    pub chunks: Vec<FileChunk>,
}

impl From<files::Model> for FileSimilarity {
    fn from(value: files::Model) -> Self {
        Self {
            id: value.id,
            title: value.title,
            path: value.path,
            chunks: Vec::new(),
        }
    }
}
