mod registration;
pub use registration::*;
mod subprocessor;

use std::{io, path::PathBuf};

use thiserror::Error;
use tokio::sync::mpsc::{error::SendError, UnboundedSender};
use walkdir::WalkDir;

#[derive(Error, Debug)]
pub enum FileIndexError {
    #[error("Couldn't send IndexEvent {0:?}")]
    ChannelError(IndexEvent),
}
impl From<SendError<IndexEvent>> for FileIndexError {
    fn from(value: SendError<IndexEvent>) -> Self {
        Self::ChannelError(value.0)
    }
}
#[derive(Debug)]
pub enum IndexEvent {
    AccessError(walkdir::Error),
    FinishedWithNoop,
    #[expect(dead_code)]
    Read {
        path: PathBuf,
        err: io::Error,
    },
    EmbeddingFailure(PathBuf),
    Register(FileRegistration),
    /// The contents of the directory have been identified and split into
    /// new async threads
    DirectoryWalked(u32),
}

/// TODO: keep a cache of already indexed files for the subprocessor to avoid
pub struct FileIndexer {
    path: PathBuf,
}

impl FileIndexer {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }
    /// MUST be called in a tokio runtime :)
    pub async fn run(self, channel: UnboundedSender<IndexEvent>) -> Result<(), FileIndexError> {
        let input = WalkDir::new(self.path);

        let mut entry_count = 0;
        for entry in input {
            let entry = match entry {
                Ok(entry) => entry,
                Err(e) => {
                    channel.send(IndexEvent::AccessError(e))?;
                    continue;
                }
            };
            entry_count += 1;
            tokio::task::spawn(subprocessor::process(entry, channel.clone()));
        }
        channel.send(IndexEvent::DirectoryWalked(entry_count))?;
        Ok(())
    }
}
