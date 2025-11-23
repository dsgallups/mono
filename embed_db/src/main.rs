mod embedder;
use candle_core::Tensor;
pub use embedder::*;

use anyhow::Result;

//mod sentence_transformer;

// using sentence-transformers/gtr-t5-base

//se burn::prelude::Backend;

fn main() -> Result<()> {
    let mut embedder = T5Embedder::new()?;
    println!("embedder loaded");

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

    let mut tensor_results = Vec::with_capacity(prompts.len());
    for prompt in prompts {
        let res = embedder.embed(prompt)?;
        tensor_results.push((prompt, res));
    }

    let mut similarities = Vec::new();

    for (i, (prmpt_one, tensor_one)) in tensor_results.iter().enumerate() {
        for (prmpt_two, tensor_two) in tensor_results.iter().skip(i + 1) {
            let similarity = similarity(tensor_one, tensor_two)?;

            similarities.push((similarity, *prmpt_one, *prmpt_two));
        }
    }

    similarities.sort_by(|a, b| (b.0.total_cmp(&a.0)));

    println!("Similarity scores:\n{similarities:#?}");

    // let similarity = similarity(&t1, &t2)?;
    // println!(r#"({similarity}): "{test_prompt}" vs "{test_prompt2}""#);
    Ok(())
}

fn similarity(t1: &Tensor, t2: &Tensor) -> Result<f32> {
    let sum_comp = (t1 * t2)?.sum_all()?.to_scalar::<f32>()?;
    let sum_t1 = (t1 * t1)?.sum_all()?.to_scalar::<f32>()?;
    let sum_t2 = (t2 * t2)?.sum_all()?.to_scalar::<f32>()?;

    let cosine_sim = sum_comp / (sum_t1 * sum_t2).sqrt();

    Ok(cosine_sim)
}
