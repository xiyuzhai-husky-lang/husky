use crate::*;
use husky_trace_protocol::SampleId;
use std::panic::{RefUnwindSafe, UnwindSafe};

pub trait LoadSample<'eval>:
    std::fmt::Debug + Send + Sync + RefUnwindSafe + UnwindSafe + 'eval
{
    fn len(&self) -> usize;
    fn load<'a>(&'a self, idx: SampleId) -> LabeledData<'eval>;
}

pub type DataLoader<'eval> = Box<dyn LoadSample<'eval>>;
