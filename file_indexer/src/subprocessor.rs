use std::{
    error::Error,
    ffi::OsStr,
    io,
    path::{Path, PathBuf},
};

use tokio::sync::mpsc::UnboundedSender;
use walkdir::DirEntry;

use crate::{IndexEvent, registration::FileRegistration};

pub async fn process(
    entry: DirEntry,
    channel: UnboundedSender<IndexEvent>,
) -> Result<(), Box<dyn Error + Send>> {
    match FileRegistration::new(entry.into_path()).await {
        Ok(registration) => {
            //recorded!
        }
        Err(err) => {
            //todo
        }
    }

    Ok(())
}
