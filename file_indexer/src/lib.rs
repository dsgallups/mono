mod registration;
pub use registration::*;
mod subprocessor;
mod tokenize;

use std::{io, path::PathBuf};

use thiserror::Error;
use tokio::sync::{
    mpsc::{UnboundedReceiver, UnboundedSender, error::SendError},
    oneshot,
};
use walkdir::WalkDir;

pub enum IndexRequest {
    Close(oneshot::Sender<Vec<IndexEvent>>),
}

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
    Read {
        path: PathBuf,
        err: io::Error,
    },
    Register(FileRegistration),
    /// The contents of the directory have been identified and split into
    /// new async threads
    DirectoryWalked,
}

/// TODO: keep a cache of already indexed files for the subprocessor to avoid
pub struct FileIndexer {
    request_chan: UnboundedReceiver<IndexRequest>,
    path: PathBuf,
}

impl FileIndexer {
    pub fn new(path: impl Into<PathBuf>, request: UnboundedReceiver<IndexRequest>) -> Self {
        Self {
            path: path.into(),
            request_chan: request,
        }
    }
    /// MUST be called in a tokio runtime :)
    pub async fn run(self, channel: UnboundedSender<IndexEvent>) -> Result<(), FileIndexError> {
        let input = WalkDir::new(self.path);

        for entry in input {
            let entry = match entry {
                Ok(entry) => entry,
                Err(e) => {
                    channel.send(IndexEvent::AccessError(e))?;
                    continue;
                }
            };
            tokio::task::spawn(subprocessor::process(entry, channel.clone()));
        }
        channel.send(IndexEvent::DirectoryWalked)?;
        Ok(())
    }
}
