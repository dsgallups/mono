use std::error::Error;

use tokio::sync::mpsc::UnboundedSender;
use walkdir::DirEntry;

use crate::IndexEvent;

pub async fn process(
    entry: DirEntry,
    channel: UnboundedSender<IndexEvent>,
) -> Result<(), Box<dyn Error + Send>> {
    Ok(())
}
