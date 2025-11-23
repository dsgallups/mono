use std::{
    path::PathBuf,
    sync::{Arc, LazyLock, Mutex},
};

use hnsw_rs::{
    api::AnnT,
    hnsw::Hnsw,
    hnswio::{HnswIo, ReloadOptions},
    prelude::DistCosine,
};

pub struct NewEmbed<'a> {
    pub id: usize,
    pub embeds: &'a [f32],
}

pub struct PromptNeighbor {
    pub id: usize,
    pub similarity: f32,
}

pub struct EmbedDb {
    reloader: Arc<Mutex<HnswIo>>,
    // hnsw: Hnsw<'static, f32, DistCosine>,
}
impl EmbedDb {
    pub fn insert(&self, embeds: Vec<NewEmbed>) {
        //a bit confused, it seems almost as if the reloader somehow ties itself to the mmap?
        let mut lock = self.reloader.lock().unwrap();
        let hnsw: Hnsw<f32, DistCosine> = lock.load_hnsw().unwrap();
        for embed in embeds {
            hnsw.insert((embed.embeds, embed.id));
        }
    }
    pub fn get(&self, embed: &[f32]) -> Vec<PromptNeighbor> {
        let mut lock = self.reloader.lock().unwrap();
        let hnsw: Hnsw<f32, DistCosine> = lock.load_hnsw().unwrap();
        let neighbors = hnsw.search(embed, 20, 50);
        let mut results = Vec::with_capacity(neighbors.len());
        for neighbor in neighbors {
            let id = neighbor.d_id;
            results.push(PromptNeighbor {
                id,
                similarity: 1. - neighbor.distance,
            });
        }

        results
    }

    pub fn save(&self) {
        let mut lock = self.reloader.lock().unwrap();
        let hnsw: Hnsw<f32, DistCosine> = lock.load_hnsw().unwrap();

        let path = PathBuf::from("vector_db");
        _ = hnsw.file_dump(&path, "doc_graph");
    }
}

impl Default for EmbedDb {
    fn default() -> Self {
        //let hnsw: Hnsw<f32, DistCosine> = Hnsw::new(32, 100000, 16, 200, DistCosine);

        let directory = PathBuf::from("vector_db");
        let mut reloader = HnswIo::new(&directory, "doc_graph");
        reloader.set_options(ReloadOptions::new(true));
        // reloader
        // reloader.set_values(directory, basename, options)

        Self {
            reloader: Arc::new(Mutex::new(reloader)),
        }
    }
}

pub static EMBED_DB: LazyLock<EmbedDb> = LazyLock::new(Default::default);
