use std::sync::{Arc, LazyLock, Mutex};

use hnsw_rs::{hnsw::Hnsw, hnswio::HnswIo, prelude::DistCosine};

pub struct NewEmbed<'a> {
    pub id: usize,
    pub embeds: &'a [f32],
}

pub struct PromptNeighbor {
    pub id: usize,
    pub similarity: f32,
}

//struct Reloader(Arc<Mutex<HnswIo>>);
// static BASE_DIR: LazyLock<PathBuf> = LazyLock::new(|| PathBuf::from("vector_db"));
// static BASE_NAME: &str = "doc_graph";

// static RELOAD: LazyLock<Reloader> = LazyLock::new(|| {
//     if fs::File::open("vector_db/doc_graph.hnsw.data").is_err() {
//         let hnsw: Hnsw<f32, DistCosine> = Hnsw::new(32, 100000, 16, 200, DistCosine);
//         hnsw.file_dump(&BASE_DIR, BASE_NAME).unwrap();
//     }

//     let mut reloader = HnswIo::new(&BASE_DIR, BASE_NAME);
//     reloader.set_options(ReloadOptions::new(false));
//     Reloader(Arc::new(Mutex::new(reloader)))
// });

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

    pub fn save(&self) {
        //self.hnsw.file_dump(&BASE_DIR, BASE_NAME).unwrap();
    }
}

impl Default for EmbedDb {
    fn default() -> Self {
        let hnsw: Hnsw<f32, DistCosine> = Hnsw::new(32, 100000, 16, 200, DistCosine);

        // let mut lock = RELOAD.0.lock().unwrap();

        // let result: Hnsw<f32, DistCosine> = lock.load_hnsw().unwrap();

        // let hnsw = unsafe {
        //     // # SAFETY: This probably isn't safe. I just got tired
        //     // of messing with the bad IO interface of `hnsw_rs`.
        //     std::mem::transmute::<Hnsw<'_, f32, DistCosine>, Hnsw<'static, f32, DistCosine>>(result)
        // };

        // reloader
        // reloader.set_values(directory, basename, options)

        Self { hnsw }
    }
}

pub static EMBED_DB: LazyLock<EmbedDb> = LazyLock::new(Default::default);
