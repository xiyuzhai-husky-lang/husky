use husky_tracer_protocol::SampleId;

use super::*;

#[derive(Debug)]
pub struct SyntheticSampleLoader<'eval> {
    len: usize,
    gen: fn(seed: u64, sample_id: SampleId) -> LabeledData<'eval>,
    seed: u64,
}

impl<'eval> SyntheticSampleLoader<'eval> {
    pub(super) fn new(
        len: usize,
        gen: fn(seed: u64, sample_id: SampleId) -> LabeledData<'eval>,
        seed: u64,
    ) -> Self {
        Self { len, gen, seed }
    }
}

impl<'eval> LoadSample<'eval> for SyntheticSampleLoader<'eval> {
    fn len(&self) -> usize {
        self.len
    }

    fn load(&self, sample_id: SampleId) -> LabeledData<'eval> {
        (self.gen)(self.seed, sample_id)
    }
}
