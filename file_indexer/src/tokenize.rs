use unicode_segmentation::UnicodeSegmentation;

/*
Running Strategy -

text files: tokenize    -> normalize    -> TF-IDF -> cosine distance -> Cluster and Rank
jpeg:                   CLIP            -> TF-IDF -> cosine-distance -> Cluster and Rank
 */
pub fn process_text(text: &str) {
    let val = UnicodeSegmentation::unicode_words(text);
}
