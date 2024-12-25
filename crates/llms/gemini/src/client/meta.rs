use super::*;
use crate::{model::GeminiModel, tier::GeminiTier};

pub struct GeminiClientMeta {
    pub api_key: String,
    pub tier: GeminiTier,
}

impl GeminiClientMeta {
    pub fn new() -> GeminiResult<Option<Self>> {
        let Some(api_key) = std::env::var("GEMINI_API_KEY").ok() else {
            return Ok(None);
        };
        let tier = match std::env::var("GEMINI_API_TIER") {
            Ok(tier) => tier.as_str().try_into()?,
            Err(_) => todo!(),
        };
        Ok(Some(Self { api_key, tier }))
    }
}
