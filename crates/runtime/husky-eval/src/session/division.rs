use husky_datasets_interface::{DataLoader, Label, LabeledData};
use husky_val_repr::*;

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
        sheets.reserve_exact(loader.len());
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

    pub fn load(&self, sample_id: SampleId) -> LabeledData<'eval> {
        self.loader.load(sample_id)
    }

    pub fn label(&self, sample_id: SampleId) -> Label {
        self.loader.label(sample_id)
    }

    pub fn each_labeled_data<'a>(&'a self) -> impl Iterator<Item = LabeledData<'eval>> + 'a {
        (0..self.loader.len())
            .into_iter()
            .map(|idx| self.loader.load(SampleId(idx)))
    }

    pub fn cache_temp_value(
        &self,
        feature: Val,
        sample_id: SampleId,
        value: &__Register<'static>,
    ) -> __Register<'static>
    where
        'eval: 'static,
    {
        let value = value.intrinsic_clone();
        self.sheets[sample_id.0]
            .cache(EvalKey::Feature(feature), Ok(value))
            .unwrap()
    }
}
