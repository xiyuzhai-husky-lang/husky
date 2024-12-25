pub mod cap;
pub mod error;
mod ext;
pub mod model;
pub mod request;
pub mod response;

use self::{error::*, request::*, response::*};
use alien_seed::{
    attach::{attached_seed, with_seed},
    AlienSeed,
};
use cap::try_call_openai;
use disk_cache::DiskCache;
use enum_index::full_map::EnumFullVecMap;
use eterned::db::EternerDb;
use lazy_static::lazy_static;
use model::OpenaiModel;
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use usage_cap::UsageCap;

pub struct OpenaiClient<'db> {
    caches: EnumFullVecMap<
        OpenaiModel,
        DiskCache<&'db EternerDb, AlienSeed, OpenaiRequest, OpenaiResponse>,
    >,
    /// None if the environment variable `OPENAI_API_KEY` is not set.
    client_ext: Option<ext::OpenAIClient>,
}

impl<'db> OpenaiClient<'db> {
    pub fn new(
        db: &'db EternerDb,
        tokio_runtime: Arc<tokio::runtime::Runtime>,
        cache_dir: &Path,
    ) -> OpenaiResult<Self> {
        let api_key = std::env::var("OPENAI_API_KEY").ok();
        Ok(Self {
            caches: EnumFullVecMap::try_new(|model: OpenaiModel| {
                DiskCache::new(
                    db,
                    tokio_runtime.clone(),
                    cache_dir.join(format!("{}.json", model.as_str())),
                )
            })?,
            client_ext: match api_key {
                Some(api_key) => Some(ext::OpenAIClient::builder().with_api_key(api_key).build()?),
                None => None,
            },
        })
    }
}

impl<'db> OpenaiClient<'db> {
    pub fn generate_text(&self, model: OpenaiModel, input: String) -> OpenaiResult<String> {
        let min_usage = input.len();
        let request = OpenaiRequest::TextGeneration { input };
        let OpenaiResponse::TextGeneration(response) = self.caches[model].get_or_call(
            attached_seed(),
            request,
            |request| -> OpenaiResult<OpenaiResponse> {
                match try_call_openai::<OpenaiResult<String>>(min_usage, || {
                    self.complete_chat_aux(model, request)
                })? {
                    Ok(result) => match result {
                        Ok(s) => Ok(OpenaiResponse::TextGeneration(s)),
                        Err(e) => Err(todo!()),
                    },
                    Err(e) => todo!(),
                }
            },
        )?;
        Ok(response)
    }

    fn complete_chat_aux(
        &self,
        model: OpenaiModel,
        request: &OpenaiRequest,
    ) -> (usize, OpenaiResult<String>) {
        let OaiRequestExt::ChatCompletion(request_ext) = request.ext(model) else {
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
        .ends_with("crates/llms/openai"));
    let cache_dir = &cargo_manifest_dir.join("caches/openai_client_works");
    assert!(
        cache_dir.exists(),
        "Cache directory does not exist at: `{}`",
        cache_dir.display()
    );
    assert!(
        cache_dir.is_dir(),
        "Path exists but is not a directory: `{}`",
        cache_dir.display()
    );

    let tokio_runtime = Arc::new(tokio::runtime::Runtime::new().unwrap());
    let client = OpenaiClient::new(db, tokio_runtime, &cache_dir).unwrap();
    let model = OpenaiModel::Gpt4o;
    let seed = AlienSeed::new(0);
    with_seed(seed, || {
        let result = client.generate_text(model, "Hello, world!".to_string());
        assert!(result.is_ok(), "{}", result.unwrap_err());
    });
}
