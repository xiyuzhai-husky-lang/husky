use crate::*;
use husky_tracer_protocol::SampleIdx;

pub trait LoadSample<'eval>: std::fmt::Debug + Send + Sync + 'eval {
    fn len(&self) -> usize;
    fn load<'a>(&'a self, idx: SampleIdx) -> LabeledData<'eval>;
}

pub type DataLoader<'eval> = Box<dyn LoadSample<'eval>>;
