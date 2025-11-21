use std::path::PathBuf;

use tokio::sync::{
    mpsc::{UnboundedReceiver, UnboundedSender},
    oneshot,
};

pub enum IndexRequest {
    Close(oneshot::Sender<Vec<IndexEvent>>),
}

pub enum IndexEvent {
    Foo(i32),
    Finished,
}

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

    pub async fn run(self, channel: UnboundedSender<IndexEvent>) {}
}
