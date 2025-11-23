use anyhow::{Error, Result};
use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::t5::{Config, T5EncoderModel};
use hf_hub::{Repo, RepoType, api::sync::Api};
use tokenizers::{EncodeInput, Tokenizer};

pub struct T5Embedder {
    tokenizer: Tokenizer,
    model: T5EncoderModel,
    device: Device,
}

impl T5Embedder {
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
        let this = T5Embedder {
            tokenizer,
            model: encoder_model,
            device,
        };
        Ok(this)
    }

    pub fn embed<'s, E>(&mut self, val: E) -> Result<Tensor>
    where
        E: Into<EncodeInput<'s>>,
    {
        println!("Here, tokenizing");
        let tokens = self.tokenizer.encode(val, true).map_err(Error::msg)?;

        let tokens = Tensor::new(tokens.get_ids(), &self.device)?.unsqueeze(0)?;

        let embeddings = self.model.forward(&tokens)?;
        println!("Embeddings: {:?}", embeddings.shape());

        let (_n_sentence, n_tokens, _hidden_size) = embeddings.dims3()?;
        let embeddings = (embeddings.sum(1)? / (n_tokens as f64))?;
        let norm_l2 = embeddings.broadcast_div(&embeddings.sqr()?.sum_keepdim(1)?.sqrt()?)?;

        println!("pooled embeddings {:?}", embeddings.shape());

        Ok(norm_l2)
    }
}
