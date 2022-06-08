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

    fn load<'a>(&'a mut self, idx: usize) -> LabeledData<'eval> {
        todo!()
    }
}
