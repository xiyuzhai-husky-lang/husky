use husky_trace_protocol_old::SampleId;

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
        50000
    }

    fn load<'a>(&'a self, sample_id: SampleId) -> LabeledData<'eval> {
        let permuted_idx = self.permutation[sample_id.0] as usize;
        let input = unsafe {
            __Register::new_temp_ref::<BinaryImage28>(
                &self.images[permuted_idx],
                &__BINARY_IMAGE_28_VTABLE,
            )
        };
        let label = self.labels[permuted_idx];
        LabeledData {
            input,
            label,
            sample_id,
        }
    }

    fn label<'a>(&'a self, sample_id: SampleId) -> Label {
        let permuted_idx = self.permutation[sample_id.0] as usize;
        self.labels[permuted_idx]
    }
}
