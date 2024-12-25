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

const DEFAULT_RETRY_DELAY_ON_FREE: std::time::Duration = std::time::Duration::from_secs(30);
const DEFAULT_RETRY_DELAY_ON_PAID: std::time::Duration = std::time::Duration::from_secs(10);

pub struct GeminiClient<'db> {
    caches: EnumFullVecMap<
        GeminiModel,
        DiskCache<&'db EternerDb, AlienSeed, GeminiRequest, GeminiResponse>,
    >,
    /// `None` means disabled
    meta: Option<GeminiClientMeta>,
    client: Client,
    retry_delay_on_free: std::time::Duration,
    retry_delay_on_paid: std::time::Duration,
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
            retry_delay_on_free: DEFAULT_RETRY_DELAY_ON_FREE,
            retry_delay_on_paid: DEFAULT_RETRY_DELAY_ON_PAID,
        })
    }
}

impl<'db> GeminiClient<'db> {
    pub fn meta(&self) -> Option<&GeminiClientMeta> {
        self.meta.as_ref()
    }

    pub fn api_key(&self) -> Option<&str> {
        self.meta.as_ref().map(|meta| meta.api_key.as_str())
    }

    pub fn tier(&self) -> Option<GeminiTier> {
        self.meta.as_ref().map(|meta| meta.tier)
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
        let min_usage = request.min_usage();
        let response = self.caches[model].get_or_call(
            attached_seed(),
            request,
            async |request| -> GeminiResult<GeminiResponse> {
                match try_call_gemini::<GeminiResult<GeminiResponse>>(
                    min_usage,
                    async || -> (usize, GeminiResult<GeminiResponse>) {
                        let Some(tier) = self.tier() else {
                            return (0, Err(GeminiError::GeminiDisabled));
                        };
                        match tier {
                            GeminiTier::Free => self.generate_on_free_aux(model, request).await,
                            GeminiTier::Paid => todo!(),
                        }
                    },
                )
                .await?
                {
                    Ok(result) => match result {
                        Ok(s) => Ok(s),
                        Err(e) => Err(e),
                    },
                    Err(e) => todo!(),
                }
            },
        )?;

        Ok(response)
    }
}
