mod error;

use error::AllLlmsResult;
use eterned::db::EternerDb;
use openai_api::OpenaiApiClient;
use std::path::PathBuf;

pub struct AllLlms<'db> {
    openai: OpenaiApiClient<'db>,
}

impl<'db> AllLlms<'db> {
    pub fn new(db: &'db EternerDb, cache_dir: PathBuf) -> AllLlmsResult<Self> {
        let oai_cache_path = cache_dir.join("openai.json");
        assert!(oai_cache_path.parent().unwrap().exists());
        Ok(Self {
            openai: OpenaiApiClient::new(db, oai_cache_path)?,
        })
    }
}
