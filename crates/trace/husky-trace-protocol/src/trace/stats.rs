use husky_signal::Signalable;

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TraceStats {
    Classification {
        samples: usize,
        arrivals: usize,
        nulls: usize,
        trues: usize,
        falses: usize,
    },
}

impl Signalable for TraceStats {}
