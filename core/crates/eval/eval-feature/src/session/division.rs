use datasets::{DataLoader, LabeledData};
use feature_gen::*;

use crate::*;

// use super::cache::EvalCache;

#[derive(Debug)]
pub struct Division<'eval> {
    loader: DataLoader<'eval>,
    pub(crate) sheets: Vec<EvalSheet<'eval>>,
    pub(crate) indicators: Vec<FeatureEvalIndicator>,
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

    pub fn load(&self, sample_idx: SampleIdx) -> LabeledData<'eval> {
        self.loader.load(sample_idx)
    }

    pub fn each_labeled_data<'a>(&'a self) -> impl Iterator<Item = LabeledData<'eval>> + 'a {
        (0..self.loader.len())
            .into_iter()
            .map(|idx| self.loader.load(SampleIdx(idx)))
    }
}
