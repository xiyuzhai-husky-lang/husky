use husky_tracer_protocol::SampleIdx;

use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MnistDevLoader {
    images: Arc<Vec<Arc<BinaryImage28>>>,
    labels: Arc<Vec<Label>>,
    permutation: Arc<Vec<u32>>,
}

impl MnistDevLoader {
    pub fn new(
        images: Arc<Vec<Arc<BinaryImage28>>>,
        labels: Arc<Vec<Label>>,
        permutation: Arc<Vec<u32>>,
    ) -> Self {
        Self {
            images,
            labels,
            permutation,
        }
    }
}

impl<'eval> LoadSample<'eval> for MnistDevLoader {
    fn len(&self) -> usize {
        10000
    }

    fn load<'a>(&'a self, sample_idx: SampleIdx) -> LabeledData<'eval> {
        let permuted_idx = self.permutation[sample_idx.0] as usize;
        let input = EvalValue::EvalPure(self.images[permuted_idx].clone());
        let label = self.labels[permuted_idx];
        LabeledData {
            input,
            label,
            sample_idx: sample_idx,
        }
    }
}
