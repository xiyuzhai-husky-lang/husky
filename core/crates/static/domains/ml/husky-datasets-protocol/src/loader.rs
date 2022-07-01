use crate::*;
use husky_trace_protocol::SampleId;

pub trait LoadSample<'eval>: std::fmt::Debug + Send + Sync + 'eval {
    fn len(&self) -> usize;
    fn load<'a>(&'a self, idx: SampleId) -> LabeledData<'eval>;
}

pub type DataLoader<'eval> = Box<dyn LoadSample<'eval>>;
