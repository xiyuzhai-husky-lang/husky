use super::*;
use sycamore::prelude::Signalable;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SampleIdx(pub usize);

impl Signalable for SampleIdx {}
