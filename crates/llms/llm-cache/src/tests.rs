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

#[test]
fn llm_cache_file_save_works() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path().join("cache.json");
    {
        let mut cache = LlmCache::<String, String>::new(path.clone()).unwrap();
        cache.get_or_call("request".to_string(), |_| "response".to_string());
    }
    {
        let cache = LlmCache::<String, String>::new(path.clone()).unwrap();
        assert_eq!(cache.entries.len(), 1);
        assert_eq!(cache.entries[0].request, "request");
        assert_eq!(cache.entries[0].response, "response");
    }
}
