use husky_signal::Signalable;

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TraceStats {
    Classification {
        dev_samples: usize,
        dev_arrivals: usize,
        dev_nulls: usize,
        dev_trues: usize,
        dev_falses: usize,
    },
}

impl Signalable for TraceStats {}
