use crate::{request::SglangRequest, response::SglangResponse, *};
use disk_cache::DiskCache;
use eterned::db::EternerDb;
use reqwest::Client;
use std::path::PathBuf;

pub struct SglangClient<'db> {
    cache: DiskCache<&'db EternerDb, SglangRequest, SglangResponse>,
    client: Client,
}

impl<'db> SglangClient<'db> {
    pub fn new(db: &'db EternerDb, file_path: PathBuf) -> SglangResult<Self> {
        Ok(Self {
            cache: DiskCache::new(db, file_path)?,
            client: Client::new(),
        })
    }
}
