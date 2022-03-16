use dataset::DataLoader;
use semantics_feature::*;

use crate::*;

// use super::cache::EvalCache;

#[derive(Debug)]
pub struct Division<'sess> {
    pub loader: DataLoader,
    pub sheets: Vec<FeatureSheet<'sess>>,
    pub indicators: Vec<FeatureEvalIndicator>,
}

impl<'sess> Division<'sess> {
    pub fn new(loader: DataLoader) -> Self {
        let mut sheets = vec![];
        let mut indicators = vec![];
        sheets.reserve(loader.len());
        (0..loader.len()).for_each(|_| {
            sheets.push(FeatureSheet::default());
            indicators.push(Default::default())
        });
        Self {
            loader,
            sheets,
            indicators,
        }
    }
}
