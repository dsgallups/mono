mod embedder;
pub use embedder::*;

use anyhow::Result;

//mod sentence_transformer;

// using sentence-transformers/gtr-t5-base

//se burn::prelude::Backend;

fn main() -> Result<()> {
    let embedder = MiniLMEmbedder::new()?;
    Ok(())
}
