use crate::{loader::LoadSample, *};

use super::*;

pub struct SyntheticSampleLoader<'a, D: SyntheticDataset> {
    dataset: &'a D,
    current: Option<Box<dyn Any>>,
}

impl<'a, D> SyntheticSampleLoader<'a, D>
where
    D: SyntheticDataset,
{
    pub(super) fn new(dataset: &'a D) -> Self {
        SyntheticSampleLoader {
            dataset,
            current: None,
        }
    }
}

impl<'a, D: SyntheticDataset> LoadSample for SyntheticSampleLoader<'a, D> {
    fn load(&mut self, idx: usize) -> &dyn Any {
        todo!()
    }
}
