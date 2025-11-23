use std::{
    ffi::OsStr,
    io,
    path::PathBuf,
    sync::{Arc, LazyLock, Mutex},
};

use embed_db::TextEmbedder;
use tokio::sync::Semaphore;

static EMBEDDER: LazyLock<Arc<Mutex<TextEmbedder>>> =
    LazyLock::new(|| Arc::new(Mutex::new(TextEmbedder::new().unwrap())));

const GIGS_ALLOWED: u64 = 8;
const BYTES_PER_PERMIT: u64 = 1 << 20;
const MAX_BYTES: u64 = GIGS_ALLOWED * (1 << 30);
const MAX_PERMITS: usize = (MAX_BYTES / BYTES_PER_PERMIT) as usize;
static OPEN_BYTES_SEM: Semaphore = Semaphore::const_new(MAX_PERMITS);

#[derive(Debug)]
pub struct FileRegistration {
    pub path: PathBuf,
    pub file_bytes: FileBytes,
}

impl FileRegistration {
    pub async fn new(path: PathBuf) -> Result<Self, FileRegError> {
        if !path.is_file() {
            return Err(FileRegError::dir(path));
        }
        // Ideally you would read the file headers here to make a determination of the file type.
        // Because I'm strapped for time, I'm only considering the extension.
        //
        // Also, nasty match pipe
        let file_bytes = match match path.extension().and_then(OsStr::to_str) {
            Some("txt") => Some(FileType::Text),
            Some("jpeg") => Some(FileType::Jpeg),
            _ => None,
        } {
            Some(file_type) => file_type.into_file_bytes(
                tokio::fs::read(&path)
                    .await
                    .map_err(|e| FileRegError::io(path.clone(), e))?,
            ),
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

#[derive(Debug)]
pub enum FileBytes {
    Text(Vec<u8>),
    Jpeg(Vec<u8>),
    Unknown,
}
impl FileBytes {}

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
