mod free;
mod meta;
mod paid;

use self::meta::*;
use crate::{request::GeminiRawRequest, response::GeminiRawResponse, *};
use alien_seed::{attach::attached_seed, AlienSeed};
use disk_cache::DiskCache;
use enum_index::full_map::EnumFullVecMap;
use eterned::db::EternerDb;
use model::GeminiModel;
use request::GeminiRequest;
use reqwest::Client;
use response::{parse_response_result, GeminiResponse};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tier::GeminiTier;
use usage_cap::UsageCap;

const DEFAULT_RETRY_DELAY: std::time::Duration = std::time::Duration::from_secs(30);

pub struct GeminiClient<'db> {
    caches: EnumFullVecMap<
        GeminiModel,
        DiskCache<&'db EternerDb, AlienSeed, GeminiRequest, GeminiResponse>,
    >,
    /// `None` means disabled
    meta: Option<GeminiClientMeta>,
    client: Client,
    retry_delay: std::time::Duration,
}

impl<'db> GeminiClient<'db> {
    pub fn new(
        db: &'db EternerDb,
        tokio_runtime: Arc<tokio::runtime::Runtime>,
        cache_dir: &Path,
    ) -> GeminiResult<Self> {
        let meta = GeminiClientMeta::new()?;
        let caches = EnumFullVecMap::try_new(|model: GeminiModel| {
            if !cache_dir.is_dir() {
                return Err(GeminiError::InvalidCacheDir(cache_dir.to_owned()));
            }
            DiskCache::new(
                db,
                tokio_runtime.clone(),
                cache_dir.join(format!("{}.json", model.as_str())),
            )
            .map_err(Into::into)
        })?;

        Ok(Self {
            caches,
            meta,
            client: Client::new(),
            retry_delay: DEFAULT_RETRY_DELAY,
        })
    }
}

impl<'db> GeminiClient<'db> {
    pub fn meta(&self) -> GeminiResult<&GeminiClientMeta> {
        self.meta.as_ref().ok_or(GeminiError::GeminiDisabled)
    }

    pub fn api_key(&self) -> GeminiResult<&str> {
        Ok(self.meta()?.api_key.as_str())
    }

    pub fn tier(&self) -> GeminiResult<GeminiTier> {
        Ok(self.meta()?.tier)
    }
}

impl<'db> GeminiClient<'db> {
    pub fn generate_text(
        &self,
        model: GeminiModel,
        input: impl Into<String>,
    ) -> GeminiResult<String> {
        match self.generate(
            model,
            GeminiRequest::TextGeneration {
                input: input.into(),
            },
        )? {
            GeminiResponse::TextGeneration { text } => Ok(text),
        }
    }

    pub fn generate(
        &self,
        model: GeminiModel,
        request: GeminiRequest,
    ) -> GeminiResult<GeminiResponse> {
        match self.meta()?.tier {
            GeminiTier::Free => self.generate_on_free(model, request),
            GeminiTier::Paid => self.generate_on_paid(model, request),
        }
    }
}
