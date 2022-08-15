use super::*;

pub struct TraceStatsKey {
    pub trace_id: TraceId,
    pub partitions: Vec<PartitionDefnData>,
}
