mod entry;
pub mod error;
#[cfg(test)]
mod tests;

use self::{
    entry::LlmCacheEntry,
    error::{LlmCacheError, LlmCacheResult},
};
use chrono::Duration;
use serde::{Deserialize, Serialize};
use std::io;
use std::{collections::HashMap, path::Path};
use std::{fs, path::PathBuf};

pub struct LlmCache<Request, Response>
where
    Request: Serialize + for<'de> Deserialize<'de> + Eq + std::hash::Hash + Clone,
    Response: Serialize + for<'de> Deserialize<'de> + Clone,
{
    path: PathBuf,
    entries: Vec<LlmCacheEntry<Request, Response>>,
    indices: HashMap<Request, usize>,
}

impl<Request, Response> LlmCache<Request, Response>
where
    Request: Serialize + for<'de> Deserialize<'de> + Eq + std::hash::Hash + Clone,
    Response: Serialize + for<'de> Deserialize<'de> + Clone,
{
    /// Creates a new LLM cache that stores request-response pairs at the specified path.
    ///
    /// # Arguments
    /// * `path` - Path where the cache file will be stored
    ///
    /// # Returns
    /// * `Ok(LlmCache)` - A new cache instance
    /// * `Err(LlmCacheError)` - If the cache file is locked or there are IO errors
    ///
    /// # Example
    /// ```
    /// use std::path::PathBuf;
    /// use llm_cache::LlmCache;
    ///
    /// let cache: LlmCache<String, String> = LlmCache::new(PathBuf::from("cache.json"))?;
    /// ```
    pub fn new(path: PathBuf) -> LlmCacheResult<Self> {
        // Create directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| LlmCacheError::Io(path.clone(), e))?;
        }

        // check lock file does not exist
        if lock_file_path(&path).exists() {
            return Err(LlmCacheError::CacheFileLockedByAnotherProcess);
        }

        // create lock file
        fs::File::create(lock_file_path(&path)).map_err(|e| LlmCacheError::Io(path.clone(), e))?;

        // Try to load existing cache
        let entries: Vec<LlmCacheEntry<Request, Response>> = if path.exists() {
            let contents =
                fs::read_to_string(&path).map_err(|e| LlmCacheError::Io(path.clone(), e))?;
            serde_json::from_str(&contents).unwrap_or_default()
        } else {
            Default::default()
        };

        let indices = entries
            .iter()
            .enumerate()
            .map(|(i, e)| (e.request.clone(), i))
            .collect();
        Ok(Self {
            path,
            entries,
            indices,
        })
    }
}

impl<Request, Response> LlmCache<Request, Response>
where
    Request: Serialize + for<'de> Deserialize<'de> + Eq + std::hash::Hash + Clone,
    Response: Serialize + for<'de> Deserialize<'de> + Clone,
{
    /// locking is handled here
    pub fn get_or_call(
        &mut self,
        request: Request,
        f: impl FnOnce(&Request) -> Response,
    ) -> LlmCacheResult<Response> {
        if let Some(index) = self.indices.get(&request) {
            Ok(self.entries[*index].response.clone())
        } else {
            let response = f(&request);
            self.entries
                .push(LlmCacheEntry::new(request.clone(), response.clone()));
            self.indices.insert(request, self.entries.len() - 1);
            Ok(response)
        }
    }

    fn save(&self) -> LlmCacheResult<()> {
        let contents = serde_json::to_string(&self.entries).unwrap();
        fs::write(&self.path, contents).map_err(|e| LlmCacheError::Io(self.path.clone(), e))
    }
}

impl<Request, Response> Drop for LlmCache<Request, Response>
where
    Request: Serialize + for<'de> Deserialize<'de> + Eq + std::hash::Hash + Clone,
    Response: Serialize + for<'de> Deserialize<'de> + Clone,
{
    fn drop(&mut self) {
        self.save().unwrap();
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
