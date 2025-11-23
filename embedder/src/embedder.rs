use anyhow::{Error, Result};
use candle_core::{DType, Device};
use candle_nn::VarBuilder;
use candle_transformers::models::t5::{Config, T5EncoderModel};
use hf_hub::{Repo, RepoType, api::sync::Api};
use tokenizers::Tokenizer;

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

        let tokenizer = Tokenizer::from_file(tokenizer).map_err(Error::msg)?;

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
}
