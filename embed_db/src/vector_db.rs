use std::sync::LazyLock;

use hnsw_rs::{hnsw::Hnsw, prelude::DistCosine};

pub struct NewEmbed<'a> {
    pub id: usize,
    pub embeds: &'a [f32],
}

pub struct PromptNeighbor {
    pub id: usize,
    pub similarity: f32,
}

pub struct EmbedDb {
    hnsw: Hnsw<'static, f32, DistCosine>,
}
impl EmbedDb {
    pub fn insert(&self, embeds: Vec<NewEmbed>) {
        //a bit confused, it seems almost as if the reloader somehow ties itself to the mmap?
        for embed in embeds {
            self.hnsw.insert((embed.embeds, embed.id));
        }
    }
    pub fn get(&self, embed: &[f32]) -> Vec<PromptNeighbor> {
        let neighbors = self.hnsw.search(embed, 20, 50);
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
}

impl Default for EmbedDb {
    fn default() -> Self {
        let hnsw: Hnsw<f32, DistCosine> = Hnsw::new(32, 100000, 16, 200, DistCosine);

        Self { hnsw }
    }
}

pub static EMBED_DB: LazyLock<EmbedDb> = LazyLock::new(Default::default);
