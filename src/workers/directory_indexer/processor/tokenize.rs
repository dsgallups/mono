use std::collections::HashMap;

use unicode_normalization::UnicodeNormalization;
use unicode_segmentation::UnicodeSegmentation;

/*
Initial idea:

text files: tokenize    -> normalize    -> TF-IDF -> cosine distance -> Cluster and Rank
jpeg:          CLIP     -> normalize    -> TF-IDF -> cosine-distance -> Cluster and Rank


Revisions

- Semantic search + chunking with BERT


Nevermind. we're going for a semantic search with an embedding with either BERT or word2vec
 */
pub fn process_text(text: &str) {
    let tokenize_and_normalize =
        UnicodeSegmentation::unicode_words(text).map(UnicodeNormalization::nfc);

    let result = tokenize_and_normalize
        .map(|normal| normal.to_string())
        .collect::<Vec<String>>();
    //
}

pub struct Tfidf {
    vocabulary: HashMap<String, usize>,
    //idf per term index
    idf: Vec<f32>,
}

impl Tfidf {
    pub fn fit(docs: &[Vec<String>]) -> Self {
        todo!()
        //let df_map: HashMap<String, usize> = HashMap::new();
    }
}
