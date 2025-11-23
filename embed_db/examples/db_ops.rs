use anyhow::Result;
use embed_db::*;
use hnsw_rs::prelude::*;

fn main() -> Result<()> {
    let prompts = [
        "The dog walked quietly down the street",
        "cats be walkin in alleys",
        "How does this even work",
        "Man I wish I could do homework right now",
        "Whose to say that I *can't* walk my dog",
        "Why are you following me",
        "My dog ate my homework",
        "This facebook post on homework excuses has gone viral",
        "I'm following this dog on instagram",
        "Let's meet up tomorrow, 9:30pm, in the alley",
    ];

    // iirc max elements is a size hint.
    let hnsw: Hnsw<f32, DistCosine> = Hnsw::new(32, 100000, 16, 200, DistCosine);
    let mut embedded = T5Embedder::new()?;
    for (id, prompt) in prompts.iter().enumerate() {
        let embed = embedded.embed(*prompt)?.squeeze(0)?;
        let floats: Vec<f32> = embed.to_vec1()?;
        hnsw.insert((floats.as_slice(), id));
    }

    let prompt_embed = embedded.embed("Social Media")?.squeeze(0)?;
    let floats: Vec<f32> = prompt_embed.to_vec1()?;
    let neighbors = hnsw.search(&floats, 100, 50);

    for neighbor in neighbors {
        let id = neighbor.d_id;

        let prompt = prompts[id];
        println!("{}, {}", 1. - neighbor.distance, prompt);
    }

    Ok(())
}
