use std::{ffi::OsStr, io, path::PathBuf};

#[derive(Debug)]
pub struct FileRegistration {
    pub path: PathBuf,
    pub file_type: FileMeta,
}

impl FileRegistration {
    pub async fn new(path: PathBuf) -> Result<Self, FileRegError> {
        if !path.is_file() {
            return Err(FileRegError::dir(path));
        }
        // Ideally you would read the file headers, check metadata, etc.
        // to make a determination of the file type.
        //
        // You would do some prepreocessing here, but I also wanted to consider that this demo
        // probably shouldn't consume all your ram.
        //
        // I might come back to this if I have time.
        //
        // I could've used a semaphore, but eh.
        let file_type = match path.extension().and_then(OsStr::to_str) {
            Some("txt") => FileMeta::Text,
            Some("jpeg") => FileMeta::Jpeg,
            _ => FileMeta::Unknown,
        };

        Ok(Self { path, file_type })
    }
}

#[derive(Debug)]
pub enum FileMeta {
    Text,
    Jpeg,
    Unknown,
}

pub struct FileRegError {
    pub path: PathBuf,
    pub err_type: FileRegErrorType,
}
impl FileRegError {
    fn dir(path: PathBuf) -> Self {
        Self {
            path,
            err_type: FileRegErrorType::Directory,
        }
    }
    #[expect(dead_code)]
    fn io(path: PathBuf, err: io::Error) -> Self {
        Self {
            path,
            err_type: FileRegErrorType::Io(err),
        }
    }
}

pub enum FileRegErrorType {
    Directory,
    Io(io::Error),
}
