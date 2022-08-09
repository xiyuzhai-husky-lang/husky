use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TraceStats {
    Classification {
        samples: usize,
        arrivals: usize,
        nulls: usize,
        trues: usize,
        falses: usize,
    },
}
