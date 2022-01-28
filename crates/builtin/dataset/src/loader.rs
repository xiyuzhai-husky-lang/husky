use crate::*;

pub trait LoadSample {
    fn len(&self) -> usize;
    fn load<'a>(&'a mut self, idx: usize) -> LabeledData;
}

pub type DataLoader = Box<dyn LoadSample>;
