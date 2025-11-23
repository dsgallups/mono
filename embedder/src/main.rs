mod embedder;
pub use embedder::*;

use anyhow::Result;

//mod sentence_transformer;

// using sentence-transformers/gtr-t5-base

//se burn::prelude::Backend;

fn main() -> Result<()> {
    let model_path = "model.safetensors";

    let embedder = GtrT5Embedder::new(model_path)?;
    Ok(())
}
