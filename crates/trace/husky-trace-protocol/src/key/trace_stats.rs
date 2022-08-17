use smallvec::SmallVec;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Hash)]
pub struct TraceStatsKey {
    pub trace_id: TraceId,
    pub partitions: Partitions,
}

const PARTITION_SMALL_VEC_SIZE: usize = 4;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Hash)]
pub struct Partitions(pub SmallVec<[PartitionDefnData; PARTITION_SMALL_VEC_SIZE]>);

impl From<SmallVec<[PartitionDefnData; PARTITION_SMALL_VEC_SIZE]>> for Partitions {
    fn from(partitions: SmallVec<[PartitionDefnData; PARTITION_SMALL_VEC_SIZE]>) -> Self {
        Self(partitions)
    }
}

impl Partitions {
    pub fn add_partition(&mut self, idx: usize, new_partition: PartitionDefnData) {
        self.0.last_mut().unwrap().ncol -= new_partition.ncol;
        self.0.insert(idx, new_partition)
    }

    pub fn label_idx(&self, label: Label) -> usize {
        self.opt_label_idx(label).unwrap_or(self.len() + 1)
    }

    pub fn opt_label_idx(&self, label: Label) -> Option<usize> {
        self.iter().position(|partition| partition.contains(label))
    }
}

impl std::ops::Deref for Partitions {
    type Target = [PartitionDefnData];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
