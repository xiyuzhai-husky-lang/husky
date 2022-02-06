use dataset::DataLoader;
use feature::FeatureSheet;

use crate::*;

// use super::cache::EvalCache;

#[derive(Debug)]
pub struct Division<'sess> {
    pub(super) loader: DataLoader,
    pub(super) caches: Vec<FeatureSheet<'sess>>,
}

impl<'sess> Division<'sess> {
    pub fn new(loader: DataLoader) -> Self {
        let mut caches = vec![];
        caches.reserve(loader.len());
        (0..loader.len()).for_each(|_| caches.push(FeatureSheet::default()));
        Self { loader, caches }
    }
}
