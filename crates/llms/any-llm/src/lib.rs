mod error;

use error::AnyLlmResult;
use openai::OaiClient;
use std::path::PathBuf;

pub struct AnyLlmClient {
    openai: OaiClient,
}

impl AnyLlmClient {
    pub fn new(cache_dir: PathBuf) -> AnyLlmResult<Self> {
        let oai_cache_path = cache_dir.join("openai.json");
        assert!(oai_cache_path.parent().unwrap().exists());
        Ok(Self {
            openai: OaiClient::new(oai_cache_path)?,
        })
    }
}
