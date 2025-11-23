use anyhow::{Error, Result};
use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::t5::{Config, T5EncoderModel};
use hf_hub::{Repo, RepoType, api::sync::Api};
use tokenizers::{EncodeInput, Tokenizer};

pub struct TextEmbedder {
    tokenizer: Tokenizer,
    model: T5EncoderModel,
    device: Device,
}

impl TextEmbedder {
    pub fn new() -> Result<Self> {
        let device = Device::new_cuda(0)?;

        let model = "t5-large";

        let rev = "main";

        let api = Api::new()?;
        let repo = api.repo(Repo::with_revision(
            model.to_string(),
            RepoType::Model,
            rev.to_string(),
        ));
        let config = repo.get("config.json")?;
        //we could use mt5 here for many languages. for the purposes of this demo, I'm going to use the english t5 tokenizer.
        let tokenizer = repo.get("tokenizer.json")?;

        let weights = repo.get("model.safetensors")?;

        let mut tokenizer = Tokenizer::from_file(tokenizer).map_err(Error::msg)?;

        tokenizer
            .with_padding(None)
            .with_truncation(None)
            .map_err(Error::msg)?;

        let vb = unsafe { VarBuilder::from_mmaped_safetensors(&[weights], DType::F32, &device)? };

        let config = std::fs::read_to_string(config)?;
        let config: Config = serde_json::from_str(&config)?;
        let encoder_model = T5EncoderModel::load(vb, &config)?;
        let this = TextEmbedder {
            tokenizer,
            model: encoder_model,
            device,
        };
        Ok(this)
    }

    pub fn chunk_embed(&mut self, val: &str) -> Result<Vec<Vec<f32>>> {
        let tokens = self.tokenizer.encode(val, true).map_err(Error::msg)?;

        // Here, we're going to basically have a chunking length of 300 tokens.
        // We'll have an overlap of around 50 tokens ~16%.
        // the stride here being 250 (chunk_len - overlap).
        // I think these params are reasonable for demo purposes.
        //
        // Ideally you'd like to chunk paragraphs and other identifiers of non-continuous thought for a
        // more reasonable strategy.
        let ids = tokens.get_ids();
        let chunk_len = 300;
        let overlap = 50;
        let stride = chunk_len - overlap;

        let mut full_chunks = Vec::with_capacity(ids.len() / chunk_len);
        let mut end_chunk = None;
        let mut start = 0;
        while start < ids.len() {
            let end = start + chunk_len;
            if end > ids.len() {
                end_chunk = Some(ids[start..ids.len()].to_vec());
                break;
            } else {
                full_chunks.push(ids[start..end].to_vec());
                start += stride;
            }
        }

        let mut embeddings = Vec::new();
        if !full_chunks.is_empty() {
            let bsz = full_chunks.len();
            let flat = full_chunks.into_iter().flatten().collect::<Vec<_>>();
            let input_ids = Tensor::from_slice(flat.as_slice(), (bsz, chunk_len), &self.device)?;
            let chunked_hidden = self.model.forward(&input_ids)?;
            let pooled = (chunked_hidden.sum(1)? / chunk_len as f64)?;
            let norm_l2 = pooled.broadcast_div(&pooled.sqr()?.sum_keepdim(1)?.sqrt()?)?;
            embeddings = norm_l2.to_vec2::<f32>()?;
        }
        if let Some(end_chunk) = end_chunk {
            let tokens = Tensor::new(end_chunk, &self.device)?.unsqueeze(0)?;

            let hidden = self.model.forward(&tokens)?;
            let (_n_sentence, n_tokens, _hidden_size) = hidden.dims3()?;
            let pool = (hidden.sum(1)? / (n_tokens as f64))?;
            let norm_l2 = pool.broadcast_div(&pool.sqr()?.sum_keepdim(1)?.sqrt()?)?;
            embeddings.push(norm_l2.squeeze(0)?.to_vec1()?);
        }

        Ok(embeddings)
    }

    pub fn naive_embed(&mut self, val: &str) -> Result<Tensor> {
        let tokens = self.tokenizer.encode(val, true).map_err(Error::msg)?;

        let tokens = Tensor::new(tokens.get_ids(), &self.device)?.unsqueeze(0)?;

        let embeddings = self.model.forward(&tokens)?;

        let (_n_sentence, n_tokens, _hidden_size) = embeddings.dims3()?;
        let embeddings = (embeddings.sum(1)? / (n_tokens as f64))?;
        let norm_l2 = embeddings.broadcast_div(&embeddings.sqr()?.sum_keepdim(1)?.sqrt()?)?;

        Ok(norm_l2)
    }
}
