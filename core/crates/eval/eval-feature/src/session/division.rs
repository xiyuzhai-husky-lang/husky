use datasets::DataLoader;
use feature_gen::*;

use crate::*;

// use super::cache::EvalCache;

#[derive(Debug)]
pub struct Division<'eval> {
    pub loader: DataLoader<'eval>,
    pub sheets: Vec<EvalSheet<'eval>>,
    pub indicators: Vec<FeatureEvalIndicator>,
}

impl<'eval> Division<'eval> {
    pub fn new(loader: DataLoader<'eval>) -> Self {
        let mut sheets = vec![];
        let mut indicators = vec![];
        sheets.reserve(loader.len());
        (0..loader.len()).for_each(|_| {
            sheets.push(EvalSheet::default());
            indicators.push(Default::default())
        });
        Self {
            loader,
            sheets,
            indicators,
        }
    }
}
