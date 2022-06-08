use crate::*;

pub trait LoadSample<'eval>: std::fmt::Debug + Send + Sync + 'eval {
    fn len(&self) -> usize;
    fn load<'a>(&'a mut self, idx: usize) -> LabeledData<'eval>;
}

pub type DataLoader<'eval> = Box<dyn LoadSample<'eval>>;
