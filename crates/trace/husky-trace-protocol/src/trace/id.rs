use super::*;
use husky_signal::Signalable;

#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Hash)]
pub struct TraceId(pub usize);

impl Signalable for TraceId {}
