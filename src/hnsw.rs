use std::sync::{Arc, LazyLock};

use hnsw_rs::{hnsw::Hnsw, prelude::DistCosine};
use tokio::sync::RwLock;

pub struct EmbedDb {
    hnsw: Arc<RwLock<Hnsw<'static, f32, DistCosine>>>,
}

impl Default for EmbedDb {
    fn default() -> Self {
        let hnsw: Hnsw<f32, DistCosine> = Hnsw::new(32, 100000, 16, 200, DistCosine);
        Self {
            hnsw: Arc::new(RwLock::new(hnsw)),
        }
    }
}

pub static EMBED_DB: LazyLock<EmbedDb> = LazyLock::new(Default::default);
