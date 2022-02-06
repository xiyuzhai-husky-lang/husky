use crate::*;

pub trait LoadSample: std::fmt::Debug + Send + Sync + 'static {
    fn len(&self) -> usize;
    fn load<'a>(&'a mut self, idx: usize) -> LabeledData;
}

pub type DataLoader = Box<dyn LoadSample>;
