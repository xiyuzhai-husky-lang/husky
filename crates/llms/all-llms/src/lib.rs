mod error;

use error::AllLlmsResult;
use eterned::db::EternerDb;
use gemini::client::GeminiClient;
use openai::OpenaiClient;
use std::path::PathBuf;

pub struct AllLlms<'db> {
    openai: OpenaiClient<'db>,
    gemini: GeminiClient<'db>,
}

impl<'db> AllLlms<'db> {
    pub fn new(db: &'db EternerDb, cache_dir: PathBuf) -> AllLlmsResult<Self> {
        let oai_cache_path = cache_dir.join("openai.json");
        let gemini_cache_path = cache_dir.join("gemini.json");
        assert!(oai_cache_path.parent().unwrap().exists());
        assert!(gemini_cache_path.parent().unwrap().exists());
        Ok(Self {
            openai: OpenaiClient::new(db, oai_cache_path)?,
            gemini: GeminiClient::new(db, gemini_cache_path)?,
        })
    }
}
