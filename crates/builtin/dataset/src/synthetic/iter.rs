use std::{
    mem::MaybeUninit,
    time::{SystemTime, UNIX_EPOCH},
};

use vm::AnyValueDyn;

use crate::iter::DataIterator;

use super::*;

pub struct SyntheticSampleIter<'a, D: SyntheticDataset> {
    dataset: &'a D,
    seed: u64,
    current: MaybeUninit<BoxedValue<'static>>,
    next_idx: usize,
}

impl<'a, D> SyntheticSampleIter<'a, D>
where
    D: SyntheticDataset,
{
    pub(super) fn new(dataset: &'a D) -> Self {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        SyntheticSampleIter {
            dataset,
            seed: since_the_epoch.as_millis() as u64,
            current: MaybeUninit::uninit(),
            next_idx: 0,
        }
    }
}

impl<'a, D: SyntheticDataset> DataIterator for SyntheticSampleIter<'a, D> {
    fn next(&mut self) -> LabeledData {
        todo!()
    }
}
