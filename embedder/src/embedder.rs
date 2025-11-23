use anyhow::{Error, Result};
use candle_core::{DType, Device};
use candle_nn::VarBuilder;
use candle_transformers::models::{
    bert::{BertModel, Config},
    t5::T5EncoderModel,
};
use hf_hub::{Repo, RepoType, api::sync::Api};
use tokenizers::Tokenizer;

pub struct T5Embedder {
    tokenizer: Tokenizer,
    model: BertModel,
    device: Device,
}

impl T5Embedder {
    pub fn new() -> Result<Self> {
        let device = Device::new_cuda(0)?;

        let model_id = "sentence-transformers/all-MiniLM-L6-v2".to_string();
        let revision = "refs/pr/21".to_string();

        let repo = Repo::with_revision(model_id, RepoType::Model, revision);
        let (config_filename, tokenizer_filename, weights_filename) = {
            let api = Api::new()?;
            let api = api.repo(repo);
            let config = api.get("config.json")?;
            let tokenizer = api.get("tokenizer.json")?;
            let weights = api.get("model.safetensors")?;

            (config, tokenizer, weights)
        };
        let config = std::fs::read_to_string(config_filename)?;
        let config: Config = serde_json::from_str(&config)?;
        let tokenizer = Tokenizer::from_file(tokenizer_filename).map_err(Error::msg)?;

        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&[weights_filename], DType::F32, &device)?
        };
        let model = BertModel::load(vb, &config)?;
        let this = Self {
            tokenizer,
            model,
            device,
        };
        Ok(this)
    }
}
