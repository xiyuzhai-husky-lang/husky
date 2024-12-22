mod entry;
pub mod error;
mod save;
#[cfg(test)]
mod tests;
pub mod traits;

use self::{
    entry::LlmCacheEntry,
    error::{DiskCacheError, DiskCacheResult},
    traits::{IsLlmCacheRequest, IsLlmCacheResponse},
};
use attach::Attach;
use chrono::Duration;
use dashmap::DashMap;
use save::LlmCacheSaveThread;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::{fs, path::PathBuf};
use std::{io, sync::RwLock};
#[cfg(test)]
use tempfile;

pub struct DiskCache<Db, Request, Response>
where
    Request: IsLlmCacheRequest,
    Response: IsLlmCacheResponse,
{
    db: Db,
    path: PathBuf,
    entries: RwLock<Vec<LlmCacheEntry<Request, Response>>>,
    indices: DashMap<Request, usize>,
    save_thread: LlmCacheSaveThread<Db, Request, Response>,
}

impl<Db, Request, Response> DiskCache<Db, Request, Response>
where
    Db: Attach,
    Request: IsLlmCacheRequest,
    Response: IsLlmCacheResponse,
{
    /// Creates a new LLM cache that stores request-response pairs at the specified path.
    ///
    /// # Arguments
    /// * `path` - Path where the cache file will be stored
    ///
    /// # Returns
    /// * `Ok(DiskCache)` - A new cache instance
    /// * `Err(LlmCacheError)` - If the cache file is locked or there are IO errors
    ///
    /// # Example
    /// ```
    /// use std::path::PathBuf;
    /// use disk_cache::DiskCache;
    ///
    /// let db = ();
    /// let temp_dir = tempfile::tempdir().unwrap();
    /// let cache_path = temp_dir.path().join("cache.json");
    /// let cache: DiskCache<(), String, String> = DiskCache::new(db, cache_path).unwrap();
    /// ```
    pub fn new(db: Db, path: PathBuf) -> DiskCacheResult<Self> {
        // Create directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| DiskCacheError::Io(path.clone(), e))?;
        }

        // check lock file does not exist
        if lock_file_path(&path).exists() {
            return Err(DiskCacheError::CacheFileLockedByAnotherProcess);
        }

        // create lock file
        fs::File::create(lock_file_path(&path)).map_err(|e| DiskCacheError::Io(path.clone(), e))?;

        // Try to load existing cache
        let entries: Vec<LlmCacheEntry<Request, Response>> = if path.exists() {
            let contents =
                fs::read_to_string(&path).map_err(|e| DiskCacheError::Io(path.clone(), e))?;
            db.attach(|| serde_json::from_str(&contents).unwrap_or_default())
        } else {
            Default::default()
        };

        let save_thread = LlmCacheSaveThread::new(db, path.clone(), entries.clone());

        let indices = entries
            .iter()
            .enumerate()
            .map(|(i, e)| (e.request.clone(), i))
            .collect();
        Ok(Self {
            db,
            path,
            entries: RwLock::new(entries),
            indices,
            save_thread,
        })
    }
}

impl<Db, Request, Response> DiskCache<Db, Request, Response>
where
    Db: Attach,
    Request: IsLlmCacheRequest,
    Response: IsLlmCacheResponse,
{
    /// locking is handled here
    pub fn get_or_call<E>(
        &self,
        request: Request,
        f: impl FnOnce(&Request) -> Result<Response, E>,
    ) -> Result<Response, E>
    where
        E: From<DiskCacheError>,
    {
        if let Some(index) = self.indices.get(&request) {
            return Ok(self.entries.read().unwrap()[*index].response.clone());
        }
        let response = self.get_or_call_aux(request, f)?;
        Ok(response)
    }

    fn get_or_call_aux<E>(
        &self,
        request: Request,
        f: impl FnOnce(&Request) -> Result<Response, E>,
    ) -> Result<Response, E>
    where
        E: From<DiskCacheError>,
    {
        let mut entries = self.entries.write().unwrap();
        // check again in case another thread has added the entry
        if let Some(index) = self.indices.get(&request) {
            return Ok(entries[*index].response.clone());
        }
        let response = f(&request)?;
        let new_entry = LlmCacheEntry::new(request.clone(), response.clone());
        entries.push(new_entry);
        self.save_thread.save(&entries)?;
        self.indices.insert(request, entries.len() - 1);
        Ok(response)
    }
}

impl<Db, Request, Response> Drop for DiskCache<Db, Request, Response>
where
    Request: IsLlmCacheRequest,
    Response: IsLlmCacheResponse,
{
    fn drop(&mut self) {
        fs::remove_file(lock_file_path(&self.path)).unwrap();
    }
}

fn lock_file_path(path: &Path) -> PathBuf {
    let ext = path.extension().unwrap_or_default().to_str().unwrap_or("");
    if ext.is_empty() {
        // No extension: just add ".lock"
        path.with_extension("lock")
    } else {
        // Has extension: append ".lock" to the existing extension
        path.with_extension(format!("{}.lock", ext))
    }
}

#[test]
fn lock_file_path_works() {
    assert_eq!(
        lock_file_path(Path::new("path/to/cache")),
        PathBuf::from("path/to/cache.lock")
    );
    assert_eq!(
        lock_file_path(Path::new("path/to/cache.json")),
        PathBuf::from("path/to/cache.json.lock")
    );
}
