use crate::*;
use tempfile::TempDir;

#[test]
fn llm_cache_lock_works() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path().join("cache.json");
    {
        let cache = LlmCache::<(), ()>::new(path.clone()).unwrap();
        assert!(lock_file_path(&cache.path).exists());
        assert!(matches!(
            LlmCache::<(), ()>::new(path.clone()),
            Err(LlmCacheError::CacheFileLockedByAnotherProcess)
        ));
    }
    assert!(!lock_file_path(&path).exists());
}
