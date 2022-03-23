use crate::loader::LoadSample;

use super::*;

#[derive(Debug)]
pub struct SyntheticSampleLoader<'eval> {
    len: usize,
    gen: fn(seed: u64, idx: usize) -> LabeledData<'eval>,
    seed: u64,
}

impl<'eval> SyntheticSampleLoader<'eval> {
    pub(super) fn new(
        len: usize,
        gen: fn(seed: u64, idx: usize) -> LabeledData<'eval>,
        seed: u64,
    ) -> Self {
        Self { len, gen, seed }
    }
}

impl<'eval> LoadSample<'eval> for SyntheticSampleLoader<'eval> {
    fn len(&self) -> usize {
        self.len
    }

    fn load(&mut self, idx: usize) -> LabeledData<'eval> {
        (self.gen)(self.seed, idx)
    }
}
