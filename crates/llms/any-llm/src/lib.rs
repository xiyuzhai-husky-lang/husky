mod error;

use error::AnyLlmResult;
use openai::OaiClient;
use std::path::PathBuf;

pub struct AnyLlmClient {
    openai: OaiClient,
}

impl AnyLlmClient {
    pub fn new(cache_dir: PathBuf) -> AnyLlmResult<Self> {
        Ok(Self {
            openai: OaiClient::new(todo!())?,
        })
    }
}
