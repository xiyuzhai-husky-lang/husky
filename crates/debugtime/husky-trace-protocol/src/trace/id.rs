use super::*;
use husky_signal::Signalable;

#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Hash)]
pub struct TraceId(pub usize);

impl std::fmt::Display for TraceId {
    fn fmt(&self, f: &mut __private::Formatter<'_>) -> std::fmt::Result {
        write!(f, "tr#{}", self.0)
    }
}

impl Signalable for TraceId {}
