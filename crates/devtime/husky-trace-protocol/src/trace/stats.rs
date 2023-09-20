use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TraceStats {
    Classification {
        dev_samples: usize,
        dev_arrivals: usize,
        dev_unreturneds: usize,
        dev_nones: usize,
        dev_trues: usize,
        dev_falses: usize,
        dev_partition_noness: Vec<(Partition, usize)>,
    },
}
