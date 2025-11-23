mod embedder;
mod vector_db;

use candle_core::Tensor;
pub use embedder::*;

use anyhow::Result;

pub fn similarity(t1: &Tensor, t2: &Tensor) -> Result<f32> {
    let sum_comp = (t1 * t2)?.sum_all()?.to_scalar::<f32>()?;
    let sum_t1 = (t1 * t1)?.sum_all()?.to_scalar::<f32>()?;
    let sum_t2 = (t2 * t2)?.sum_all()?.to_scalar::<f32>()?;

    let cosine_sim = sum_comp / (sum_t1 * sum_t2).sqrt();

    Ok(cosine_sim)
}
