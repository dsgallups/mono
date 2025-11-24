use std::{convert::Infallible, fmt::Display, str::FromStr};

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
    pub id: i32,
    pub content: String,
    pub similarity: f32,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FileType {
    Text,
    Jpeg,
    Unknown,
}
impl Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileType::Jpeg => write!(f, "jpeg"),
            FileType::Text => write!(f, "text"),
            FileType::Unknown => write!(f, "unknown"),
        }
    }
}

impl FromStr for FileType {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "jpeg" => Ok(Self::Jpeg),
            "text" => Ok(Self::Text),
            _ => Ok(Self::Unknown),
        }
    }
}

#[derive(Serialize)]
pub struct FileSimilarity {
    pub id: i32,
    pub title: String,
    pub file_type: FileType,
    pub path: String,
    pub chunks: Vec<FileChunk>,
}

impl From<files::Model> for FileSimilarity {
    fn from(value: files::Model) -> Self {
        let Ok(file_type) = FileType::from_str(&value.file_type);
        Self {
            id: value.id,
            title: value.title,
            path: value.path,
            file_type,
            chunks: Vec::new(),
        }
    }
}
