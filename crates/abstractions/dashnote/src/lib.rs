//! Thread-safe memoization caches built on top of `DashMap`.
//!
//! This crate provides two main types for concurrent memoization:
//!
//! - [`DashNote`]: A cache for values that can be copied
//! - [`DashRefNote`]: A cache that returns references to stored values
//!
//! # Example
//! ```
//! use dashnote::DashNote;
//!
//! let cache: DashNote<i32, i32> = DashNote::default();
//! let value = cache.get_or_calc(1, || 42);
//! assert_eq!(value, 42);
//! ```
//!
//! # Features
//! - Thread-safe concurrent access
//! - Automatic computation and caching of missing values
//! - Choice between copied values ([`DashNote`]) or references ([`DashRefNote`])

use dashmap::DashMap;

/// A concurrent memoization cache that stores computed values by key.
///
/// `DashNote` provides thread-safe caching of values that can be copied.
/// It uses `DashMap` internally for concurrent access.
///
/// # Type Parameters
/// - `K`: The key type that must implement `Hash`, `Eq` and `Copy`
/// - `V`: The value type that must implement `Copy`
pub struct DashNote<K, V>
where
    K: std::hash::Hash + Eq + Copy,
{
    map: DashMap<K, V>,
}

impl<K, V> Default for DashNote<K, V>
where
    K: std::hash::Hash + Eq + Copy,
{
    fn default() -> Self {
        Self {
            map: DashMap::default(),
        }
    }
}

impl<K, V> DashNote<K, V>
where
    K: std::hash::Hash + Eq + Copy,
{
    /// Retrieves a value from the cache by key, computing and storing it if not present.
    ///
    /// # Arguments
    /// * `key` - The key to look up
    /// * `f` - A function that computes the value if not found in cache
    ///
    /// # Returns
    /// A copy of the value associated with the key
    pub fn get_or_calc(&self, key: K, f: impl FnOnce() -> V) -> V
    where
        V: Copy,
    {
        self.map.entry(key).or_insert_with(f).value().clone()
    }
}

/// A concurrent memoization cache that stores computed values by key and returns references.
///
/// Similar to `DashNote`, but stores boxed values and returns references instead of copies.
/// This is useful for large values that are expensive to copy.
///
/// # Type Parameters
/// - `K`: The key type that must implement `Hash`, `Eq` and `Copy`
/// - `V`: The value type
pub struct DashRefNote<K, V> {
    map: DashMap<K, Box<V>>,
}

impl<K, V> Default for DashRefNote<K, V>
where
    K: std::hash::Hash + Eq + Copy,
{
    fn default() -> Self {
        Self {
            map: DashMap::default(),
        }
    }
}

impl<K, V> DashRefNote<K, V>
where
    K: std::hash::Hash + Eq + Copy,
{
    /// Retrieves a reference to a value from the cache by key, computing and storing it if not present.
    ///
    /// # Arguments
    /// * `key` - The key to look up
    /// * `f` - A function that computes the value if not found in cache
    ///
    /// # Returns
    /// A reference to the value associated with the key
    ///
    /// # Safety
    /// Uses unsafe code internally to extend the lifetime of the returned reference
    pub fn get_or_calc(&self, key: K, f: impl FnOnce() -> V) -> &V {
        use husky_wild_utils::arb_ref;

        let entry = self.map.entry(key).or_insert_with(|| Box::new(f()));
        let v: &V = &**entry.value();
        unsafe { arb_ref(v) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::thread;

    #[test]
    fn test_dashnote_basic() {
        let cache: DashNote<i32, i32> = DashNote::default();

        // Test single value
        let value = cache.get_or_calc(1, || 42);
        assert_eq!(value, 42);

        // Test cached value is returned
        let counter = AtomicUsize::new(0);
        let value = cache.get_or_calc(1, || {
            counter.fetch_add(1, Ordering::SeqCst);
            100
        });
        assert_eq!(value, 42);
        assert_eq!(counter.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn test_dashrefnote_basic() {
        let cache: DashRefNote<i32, String> = DashRefNote::default();

        // Test single value
        let value = cache.get_or_calc(1, || "one".to_string());
        assert_eq!(value, "one");

        // Test cached value is returned
        let counter = AtomicUsize::new(0);
        let value = cache.get_or_calc(1, || {
            counter.fetch_add(1, Ordering::SeqCst);
            "different".to_string()
        });
        assert_eq!(value, "one");
        assert_eq!(counter.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn test_concurrent_access() {
        let cache = std::sync::Arc::new(DashNote::default());

        // Spawn multiple threads accessing the same key
        let handles: Vec<_> = (0..10)
            .map(|_| {
                let cache = cache.clone();
                thread::spawn(move || cache.get_or_calc(1, || 42))
            })
            .collect();

        // All threads should get the same value
        for handle in handles {
            assert_eq!(handle.join().unwrap(), 42);
        }
    }

    #[test]
    fn test_multiple_keys() {
        let cache = DashRefNote::default();

        let value1 = cache.get_or_calc(1, || "one".to_string());
        let value2 = cache.get_or_calc(2, || "two".to_string());

        assert_eq!(value1, "one");
        assert_eq!(value2, "two");

        // Verify values are cached
        let value1_again = cache.get_or_calc(1, || panic!("Should not be called"));
        assert_eq!(value1_again, "one");
    }
}
