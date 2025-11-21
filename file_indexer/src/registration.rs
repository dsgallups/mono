use std::{ffi::OsStr, io, path::PathBuf};

pub struct FileRegistration {
    path: PathBuf,
    file_bytes: FileBytes,
}

impl FileRegistration {
    pub async fn new(path: PathBuf) -> Result<Self, FileRegError> {
        if !path.is_file() {
            return Err(FileRegError::Directory);
        }
        // Ideally you would read the file headers here to make a determination of the file type.
        // Because I'm strapped for time, I'm only considering the extension.

        let file_bytes = match match path.extension().and_then(OsStr::to_str) {
            Some("txt") => Some(FileType::Text),
            Some("jpeg") => Some(FileType::Jpeg),
            _ => None,
        } {
            Some(file_type) => {
                file_type.into_file_bytes(tokio::fs::read(&path).await.map_err(FileRegError::Io)?)
            }
            None => {
                return Ok(Self {
                    path,
                    file_bytes: FileBytes::Unknown,
                });
            }
        };

        Ok(Self { path, file_bytes })
    }
}

enum FileType {
    Text,
    Jpeg,
}
impl FileType {
    pub fn into_file_bytes(self, bytes: Vec<u8>) -> FileBytes {
        match self {
            FileType::Text => FileBytes::Text(bytes),
            FileType::Jpeg => FileBytes::Jpeg(bytes),
        }
    }
}

pub enum FileBytes {
    Text(Vec<u8>),
    Jpeg(Vec<u8>),
    Unknown,
}
impl FileBytes {}

pub enum FileRegError {
    Directory,
    Io(io::Error),
}
