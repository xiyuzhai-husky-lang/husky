use crate::loader::LoadSample;

use super::*;

pub struct SyntheticSampleLoader {
    len: usize,
    gen: fn(seed: u64, idx: usize) -> LabeledData,
    seed: u64,
}

impl SyntheticSampleLoader {
    pub(super) fn new(
        len: usize,
        gen: fn(seed: u64, idx: usize) -> LabeledData,
        seed: u64,
    ) -> Self {
        Self { len, gen, seed }
    }
}

impl LoadSample for SyntheticSampleLoader {
    fn len(&self) -> usize {
        self.len
    }

    fn load(&mut self, idx: usize) -> LabeledData {
        (self.gen)(self.seed, idx)
    }
}
