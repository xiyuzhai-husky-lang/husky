use super::*;
use husky_signal::Signalable;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SampleId(pub usize);

impl Signalable for SampleId {}
