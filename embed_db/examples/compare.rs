use embed_db::{T5Embedder, similarity};

fn main() -> anyhow::Result<()> {
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

    similarities.sort_by(|a, b| b.0.total_cmp(&a.0));

    println!("Similarity scores:");
    for (score, p1, p2) in similarities.into_iter().take(10) {
        println!(r#"({score}): "{p1}" vs. "{p2}""#);
    }

    Ok(())
}
