use serde::Serialize;

use crate::models::files;

#[derive(Serialize)]
pub struct FileResponse {
    id: i32,
    title: String,
}

impl From<files::Model> for FileResponse {
    fn from(value: files::Model) -> Self {
        Self {
            id: value.id,
            title: value.title,
        }
    }
}
