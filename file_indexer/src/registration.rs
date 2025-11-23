use std::{
    ffi::OsStr,
    io,
    path::PathBuf,
    sync::{Arc, LazyLock, Mutex},
};

use embed_db::TextEmbedder;
use tokio::{fs, sync::Semaphore};

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
    pub file_bytes: FileEmbeddings,
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
        let file_type = match path.extension().and_then(OsStr::to_str) {
            Some("txt") => Some(FileType::Text),
            Some("jpeg") => Some(FileType::Jpeg),
            _ => None,
        };

        let Some(file_type) = file_type else {
            return Ok(Self {
                path,
                file_bytes: FileEmbeddings::Unknown,
            });
        };

        let mut _permit = None;

        match fs::metadata(&path).await {
            Ok(metadata) => {
                let length = metadata.len();
                let needed = length.div_ceil(BYTES_PER_PERMIT);
                _permit = Some(OPEN_BYTES_SEM.acquire_many(needed as u32).await.unwrap());
            }
            Err(e) => {
                return Err(FileRegError::io(path, e));
            }
        }
        // We will process entire files altogether. There is probably
        // a better implementation with buffering, but I assumed in my
        // embedder that all file chunks are available instantly.
        let prompt = match file_type {
            FileType::Text => fs::read_to_string(&path)
                .await
                .map_err(|e| FileRegError::io(path.clone(), e))?,
            FileType::Jpeg => {
                todo!("Will use CLIP on JPEGs")
            }
        };

        let embeddings = {
            let mut embedder = EMBEDDER.lock().unwrap();
            embedder
                .chunk_embed(&prompt)
                .map_err(|_e| FileRegError::embedding(path.clone()))?
        };

        let file_embeddings = file_type.into_file_bytes(embeddings);

        Ok(Self {
            path,
            file_bytes: file_embeddings,
        })
    }
}

#[derive(Clone, Copy)]
enum FileType {
    Text,
    Jpeg,
}
impl FileType {
    pub fn into_file_bytes(self, embeddings: Vec<Vec<f32>>) -> FileEmbeddings {
        match self {
            FileType::Text => FileEmbeddings::Text(embeddings),
            FileType::Jpeg => FileEmbeddings::Jpeg(embeddings),
        }
    }
}

#[derive(Debug)]
pub enum FileEmbeddings {
    Text(Vec<Vec<f32>>),
    Jpeg(Vec<Vec<f32>>),
    Unknown,
}
impl FileEmbeddings {}

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

    fn embedding(path: PathBuf) -> Self {
        Self {
            path,
            err_type: FileRegErrorType::Embedding,
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
    Embedding,
    Io(io::Error),
}
