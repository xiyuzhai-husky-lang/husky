use husky_trace_protocol::SampleId;

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

    fn load<'a>(&'a self, sample_id: SampleId) -> LabeledData<'eval> {
        let permuted_idx = self.permutation[sample_id.0] as usize;
        let input = unsafe {
            __Register::new_temp_ref(
                &self.images[permuted_idx] as &BinaryImage28,
                &__BINARY_IMAGE28_VTABLE,
            )
        };
        let label = self.labels[permuted_idx];
        LabeledData {
            input,
            label,
            sample_id: sample_id,
        }
    }
}
