use super::*;
use dashmap::DashMap;

pub struct Jar<K, T> {
    pub data: DashMap<K, Pin<Box<T>>>,
}

impl<K, T> Jar<K, T>
where
    K: Eq + std::hash::Hash,
{
    pub fn get_or_alloc(&self, key: K, f: impl FnOnce() -> T) -> &T {
        use husky_wild_utils::arb_ref;

        if let Some(t) = self.data.get(&key) {
            return unsafe { arb_ref(&*t) };
        }

        // We have to do in this way to avoid race conditions.
        // In `f()`, the same dashmap might be used by the same thread, causing deadlock.
        // We have to let go of the lock before calling `f()`.
        // Although, we might end up calling `f()` multiple times, but that's better than deadlocking.
        let t = f();
        let pin = Box::pin(t);
        let t = unsafe { arb_ref(&*pin) };
        self.data.insert(key, pin);
        t
    }
}

impl<K, T> Default for Jar<K, T>
where
    K: Eq + std::hash::Hash,
{
    fn default() -> Self {
        Self {
            data: DashMap::new(),
        }
    }
}

#[sealed]
impl<K, T> IsNoteJar for Jar<K, T>
where
    K: Eq + std::hash::Hash + Send + Sync + 'static,
    T: Send + Sync + 'static,
{
}

#[sealed]
impl<K, T> IsNoteJarDyn for Jar<K, T>
where
    K: Eq + std::hash::Hash + Send + Sync + 'static,
    T: Send + Sync + 'static,
{
}
