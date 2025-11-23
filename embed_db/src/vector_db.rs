use std::{path::PathBuf, sync::LazyLock};

use hnsw_rs::{api::AnnT, hnsw::Hnsw, hnswio::HnswIo, prelude::DistCosine};

pub struct NewEmbed<'a> {
    pub id: usize,
    pub embeds: &'a [f32],
}

pub struct EmbedDb {
    hnsw: Hnsw<'static, f32, DistCosine>,
}
impl EmbedDb {
    pub fn insert(&self, embeds: Vec<NewEmbed>) {
        for embed in embeds {
            self.hnsw.insert((embed.embeds, embed.id));
        }
    }

    pub fn save(&self) {
        let path = PathBuf::from("vector_db");
        _ = self.hnsw.file_dump(&path, "doc_graph");
    }
}

impl Default for EmbedDb {
    fn default() -> Self {
        let hnsw: Hnsw<f32, DistCosine> = Hnsw::new(32, 100000, 16, 200, DistCosine);

        // let directory = PathBuf::from("vector_db");
        // let mut reloader = HnswIo::new(&directory, "doc_graph");
        // let result = reloader.load_hnsw::<f32, DistCosine>().unwrap();

        Self { hnsw }
    }
}

pub static EMBED_DB: LazyLock<EmbedDb> = LazyLock::new(Default::default);
