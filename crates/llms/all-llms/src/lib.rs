mod error;
pub mod model;
#[cfg(test)]
mod tests;

use error::{AllLlmsResult, AnyLlmError};
use eterned::db::EternerDb;
use gemini::client::GeminiClient;
use model::AllLlmModel;
use openai::OpenaiClient;
use std::path::PathBuf;

pub struct AllLlmsClient<'db> {
    openai: OpenaiClient<'db>,
    gemini: GeminiClient<'db>,
}

impl<'db> AllLlmsClient<'db> {
    pub fn new(db: &'db EternerDb, cache_dir: PathBuf) -> AllLlmsResult<Self> {
        assert!(cache_dir.exists());
        assert!(cache_dir.is_dir());
        let openai_cache_dir = &cache_dir.join("openai");
        if !openai_cache_dir.exists() {
            std::fs::create_dir_all(openai_cache_dir)
                .map_err(|e| AnyLlmError::InvalidCacheDir(e))?;
        }
        if !openai_cache_dir.is_dir() {
            todo!()
            // return Err(AllLlmsError::InvalidCacheDir(gemini_cache_dir.to_owned()));
        }
        let gemini_cache_dir = &cache_dir.join("gemini");
        if !gemini_cache_dir.exists() {
            std::fs::create_dir_all(gemini_cache_dir)
                .map_err(|e| AnyLlmError::InvalidCacheDir(e))?;
        }
        if !gemini_cache_dir.is_dir() {
            todo!()
            // return Err(AllLlmsError::InvalidCacheDir(gemini_cache_dir.to_owned()));
        }
        Ok(Self {
            openai: OpenaiClient::new(db, openai_cache_dir)?,
            gemini: GeminiClient::new(db, gemini_cache_dir)?,
        })
    }
}

impl<'db> AllLlmsClient<'db> {
    pub fn generate_text(&self, model: AllLlmModel, prompt: String) -> AllLlmsResult<String> {
        match model {
            AllLlmModel::Openai(openai_model) => self
                .openai
                .generate_text(openai_model, prompt)
                .map_err(Into::into),
            AllLlmModel::Gemini(gemini_model) => self
                .gemini
                .generate_text(gemini_model, prompt)
                .map_err(Into::into),
            AllLlmModel::Sglang(sglang_model) => todo!(),
        }
    }
}
