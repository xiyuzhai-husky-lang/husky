use crate::{request::SglangRequest, response::SglangResponse, *};
use alien_seed::AlienSeed;
use disk_cache::DiskCache;
use enum_index::full_map::EnumFullVecMap;
use eterned::db::EternerDb;
use model::SglangModel;
use reqwest::Client;
use std::path::PathBuf;
use std::sync::Arc;

pub struct SglangClient<'db> {
    caches: EnumFullVecMap<
        SglangModel,
        DiskCache<&'db EternerDb, AlienSeed, SglangRequest, SglangResponse>,
    >,
    client: Client,
}

impl<'db> SglangClient<'db> {
    pub fn new(
        db: &'db EternerDb,
        tokio_runtime: Arc<tokio::runtime::Runtime>,
        cache_dir: PathBuf,
    ) -> SglangResult<Self> {
        if !cache_dir.is_dir() {
            return Err(SglangError::InvalidCacheDir(cache_dir));
        }

        let caches = EnumFullVecMap::try_new(|model: SglangModel| {
            if !cache_dir.is_dir() {
                return Err(SglangError::InvalidCacheDir(cache_dir.clone()));
            }
            DiskCache::new(db, tokio_runtime.clone(), cache_dir.join(model.as_str()))
                .map_err(Into::into)
        })?;
        let client = Client::new();
        Ok(Self { caches, client })
    }
}
