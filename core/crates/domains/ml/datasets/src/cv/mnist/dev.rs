use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MnistDevLoader {
    images: Arc<Vec<Arc<BinaryImage28>>>,
    labels: Arc<Vec<u8>>,
    permutation: Arc<Vec<u32>>,
}

impl MnistDevLoader {
    pub fn new(
        images: Arc<Vec<Arc<BinaryImage28>>>,
        labels: Arc<Vec<u8>>,
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

    fn load<'a>(&'a mut self, idx: usize) -> LabeledData<'eval> {
        let permuted_idx = self.permutation[idx] as usize;
        let input = EvalValue::EvalPure(self.images[permuted_idx].clone());
        let label = self.labels[permuted_idx];
        LabeledData { input, label }
    }
}
