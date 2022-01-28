use dataset::DataLoader;

use crate::*;

use super::cache::EvalCache;

pub struct Division<'sess> {
    pub(super) loader: DataLoader,
    pub(super) caches: Vec<EvalCache<'sess>>,
}

impl<'sess> Division<'sess> {
    pub fn new(loader: DataLoader) -> Self {
        let mut caches = vec![];
        caches.reserve(loader.len());
        (0..loader.len()).for_each(|_| caches.push(EvalCache::default()));
        Self { loader, caches }
    }
}
