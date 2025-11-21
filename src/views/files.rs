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
