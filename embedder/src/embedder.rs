use anyhow::Result;
use candle_core::Device;
use candle_transformers::models::t5::T5EncoderModel;
use tokenizers::Tokenizer;

pub struct GtrT5Embedder {
    tokenizer: Tokenizer,
    model: T5EncoderModel,
    device: Device,
}

impl GtrT5Embedder {
    pub fn new(path: &str) -> Result<Self> {
        let device = Device::new_cuda(0)?;

        todo!()
    }
}
