use tokio::sync::mpsc::UnboundedSender;
use walkdir::DirEntry;

use crate::workers::directory_indexer::{
    processor::{FileRegError, FileRegErrorType, NewFileRegistration},
    FileIndexError, IndexEvent,
};

pub async fn process(
    entry: DirEntry,
    channel: UnboundedSender<IndexEvent>,
) -> Result<(), FileIndexError> {
    match match NewFileRegistration::new(entry.into_path()).await {
        Ok(registration) => {
            channel.send(IndexEvent::Record(registration.clone()))?;
            Some(registration.into_file_registration().await)
        }
        Err(e) => {
            handle_reg_error(&channel, e)?;
            None
        }
    } {
        Some(Ok(result)) => {
            channel.send(IndexEvent::Register(result))?;
        }
        Some(Err(e)) => handle_reg_error(&channel, e)?,
        None => {}
    }

    Ok(())
}

fn handle_reg_error(
    channel: &UnboundedSender<IndexEvent>,
    err: FileRegError,
) -> Result<(), FileIndexError> {
    let FileRegError { path, err_type } = err;
    match err_type {
        FileRegErrorType::Directory => {
            channel.send(IndexEvent::FinishedWithNoop)?;
        }
        FileRegErrorType::Embedding => {
            channel.send(IndexEvent::EmbeddingFailure(path))?;
        }
        FileRegErrorType::Io(err) => {
            channel.send(IndexEvent::Read { path, err })?;
        }
    }
    Ok(())
}
