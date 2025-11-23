use std::{ffi::OsStr, io, path::PathBuf};

use embed_db::{Chunk, EMBEDDER};
use tokio::{fs, sync::Semaphore};
use tracing::info;

const GIGS_ALLOWED: u64 = 8;
const BYTES_PER_PERMIT: u64 = 1 << 20;
const MAX_BYTES: u64 = GIGS_ALLOWED * (1 << 30);
const MAX_PERMITS: usize = (MAX_BYTES / BYTES_PER_PERMIT) as usize;
static OPEN_BYTES_SEM: Semaphore = Semaphore::const_new(MAX_PERMITS);

#[derive(Debug, Clone)]
pub struct NewFileRegistration {
    pub path: PathBuf,
    pub file_type: Option<(FileType, u64)>,
}

impl NewFileRegistration {
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
                file_type: None,
            });
        };

        let required_permits = match fs::metadata(&path).await {
            Ok(metadata) => {
                let length = metadata.len();
                length.div_ceil(BYTES_PER_PERMIT)
            }
            Err(e) => {
                return Err(FileRegError::io(path, e));
            }
        };

        Ok(Self {
            path,
            file_type: Some((file_type, required_permits)),
        })
    }

    pub async fn into_file_registration(self) -> Result<FileRegistration, FileRegError> {
        let Some((file_type, required_permits)) = self.file_type else {
            return Ok(FileRegistration {
                path: self.path,
                contents: FileEmbeddings::Unknown,
            });
        };

        let _permit = OPEN_BYTES_SEM
            .acquire_many(required_permits as u32)
            .await
            .unwrap();

        // We will process entire files altogether. There is probably
        // a better implementation with buffering, but I assumed in my
        // embedder that all file chunks are available instantly.
        let prompt = match file_type {
            FileType::Text => fs::read_to_string(&self.path)
                .await
                .map_err(|e| FileRegError::io(self.path.clone(), e))?,
            FileType::Jpeg => {
                todo!("Will use CLIP on JPEGs")
            }
        };

        let embeddings = {
            let mut embedder = EMBEDDER.lock().unwrap();
            info!("({:?}): Processing embeddings", self.path);
            let res = embedder
                .chunk_embed(&prompt)
                .map_err(|_e| FileRegError::embedding(self.path.clone()))?;
            info!("({:?}): Processed!", self.path);
            res
        };

        let file_embeddings = file_type.into_file_bytes(embeddings);

        Ok(FileRegistration {
            path: self.path,
            contents: file_embeddings,
        })
    }
}

#[derive(Debug)]
pub struct FileRegistration {
    pub path: PathBuf,
    pub contents: FileEmbeddings,
}

#[derive(Clone, Copy, Debug)]
enum FileType {
    Text,
    Jpeg,
}
impl FileType {
    pub fn into_file_bytes(self, embeddings: Vec<Chunk>) -> FileEmbeddings {
        match self {
            FileType::Text => FileEmbeddings::Text(embeddings),
            FileType::Jpeg => FileEmbeddings::Jpeg(embeddings),
        }
    }
}

#[derive(Debug)]
pub enum FileEmbeddings {
    Text(Vec<Chunk>),
    Jpeg(Vec<Chunk>),
    Unknown,
}
impl FileEmbeddings {
    pub fn chunks(&self) -> Option<&[Chunk]> {
        match self {
            FileEmbeddings::Jpeg(v) | FileEmbeddings::Text(v) => Some(v.as_slice()),
            FileEmbeddings::Unknown => None,
        }
    }
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
