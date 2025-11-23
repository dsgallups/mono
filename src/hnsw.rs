use std::sync::LazyLock;

use hnsw_rs::{hnsw::Hnsw, prelude::DistCosine};

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
}

impl Default for EmbedDb {
    fn default() -> Self {
        let hnsw: Hnsw<f32, DistCosine> = Hnsw::new(32, 100000, 16, 200, DistCosine);
        Self { hnsw }
    }
}

pub static EMBED_DB: LazyLock<EmbedDb> = LazyLock::new(Default::default);
