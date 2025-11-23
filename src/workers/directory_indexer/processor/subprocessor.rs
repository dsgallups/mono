use tokio::sync::mpsc::UnboundedSender;
use walkdir::DirEntry;

use crate::workers::directory_indexer::{
    processor::{FileRegError, FileRegErrorType, FileRegistration},
    FileIndexError, IndexEvent,
};

pub async fn process(
    entry: DirEntry,
    channel: UnboundedSender<IndexEvent>,
) -> Result<(), FileIndexError> {
    match FileRegistration::new(entry.into_path()).await {
        Ok(registration) => {
            channel.send(IndexEvent::Register(registration))?;
        }
        Err(FileRegError { path, err_type }) => match err_type {
            FileRegErrorType::Directory => {
                channel.send(IndexEvent::FinishedWithNoop);
            }
            FileRegErrorType::Embedding => {
                channel.send(IndexEvent::EmbeddingFailure(path))?;
            }
            FileRegErrorType::Io(err) => {
                channel.send(IndexEvent::Read { path, err })?;
            }
        },
    }

    Ok(())
}
