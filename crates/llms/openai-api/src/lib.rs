pub mod cap;
pub mod error;
mod ext;
pub mod request;
pub mod response;

use self::{error::*, request::*, response::*};
use cap::try_call_openai;
use disk_cache::DiskCache;
use eterned::db::EternerDb;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use usage_cap::UsageCap;

pub struct OpenaiClient<'db> {
    cache: DiskCache<&'db EternerDb, OpenaiRequest, OpenaiResponse>,
    /// None if the environment variable `OPENAI_API_KEY` is not set.
    client_ext: Option<ext::OpenAIClient>,
}

impl<'db> OpenaiClient<'db> {
    pub fn new(db: &'db EternerDb, file_path: PathBuf) -> OpenaiResult<Self> {
        let api_key = std::env::var("OPENAI_API_KEY").ok();
        Ok(Self {
            cache: DiskCache::new(db, file_path)?,
            client_ext: match api_key {
                Some(api_key) => Some(ext::OpenAIClient::builder().with_api_key(api_key).build()?),
                None => None,
            },
        })
    }
}

impl<'db> OpenaiClient<'db> {
    pub fn complete_chat(&self, prompt: String) -> OpenaiResult<String> {
        let min_usage = prompt.len();
        let request = OpenaiRequest::ChatCompletion(prompt);
        let OpenaiResponse::ChatCompletion(response) =
            self.cache
                .get_or_call(request, |request| -> OpenaiResult<OpenaiResponse> {
                    match try_call_openai::<OpenaiResult<String>>(min_usage, || {
                        self.complete_chat_aux(request)
                    })? {
                        Ok(result) => match result {
                            Ok(s) => Ok(OpenaiResponse::ChatCompletion(s)),
                            Err(e) => Err(todo!()),
                        },
                        Err(e) => todo!(),
                    }
                })?;
        Ok(response)
    }

    fn complete_chat_aux(&self, request: &OpenaiRequest) -> (usize, OpenaiResult<String>) {
        let OaiRequestExt::ChatCompletion(request_ext) = request.ext() else {
            unreachable!()
        };
        match self.complete_chat_ext(request_ext) {
            Ok(content) => (content.len(), Ok(content)),
            Err(e) => (0, Err(e)), // ad hoc
        }
    }
}

#[test]
fn openai_client_works() {
    let db = &EternerDb::default();

    let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    assert!(cargo_manifest_dir
        .canonicalize()
        .unwrap()
        .ends_with("crates/llms/openai-api"));
    let cache_path = cargo_manifest_dir.join("cache/openai_client_works.json");
    assert!(cache_path.exists());

    let client = OpenaiClient::new(db, cache_path).unwrap();
    let result = client.complete_chat("Hello, world!".to_string());
    assert!(result.is_ok(), "{}", result.unwrap_err());
}
