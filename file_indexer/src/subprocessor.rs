use tokio::sync::mpsc::UnboundedSender;
use walkdir::DirEntry;

use crate::{
    FileIndexError, IndexEvent,
    registration::{FileRegError, FileRegErrorType, FileRegistration},
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
            FileRegErrorType::Directory => {}
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
