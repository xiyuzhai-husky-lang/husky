use husky_trace_protocol_old::SampleId;

use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MnistValLoader {
    permutation: Arc<Vec<u32>>,
}

impl MnistValLoader {
    pub fn new(permutation: Arc<Vec<u32>>) -> Self {
        Self { permutation }
    }
}

impl<'eval> LoadSample<'eval> for MnistValLoader {
    fn len(&self) -> usize {
        10000
    }

    fn load<'a>(&'a self, _sample_id: SampleId) -> LabeledData<'eval> {
        todo!()
    }

    fn label<'a>(&'a self, _idx: SampleId) -> Label {
        todo!()
    }
}
