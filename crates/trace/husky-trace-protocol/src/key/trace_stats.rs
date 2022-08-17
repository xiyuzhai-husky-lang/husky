use smallvec::SmallVec;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Hash)]
pub struct TraceStatsKey {
    pub trace_id: TraceId,
    pub partitions: Partitions,
}

const PARTITION_SMALL_VEC_SIZE: usize = 4;
const NCOL_TOTAL: u32 = 7;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Hash, Default)]
pub struct Partitions {
    nondefaults: SmallVec<[PartitionDefnData; PARTITION_SMALL_VEC_SIZE]>,
    default_partition_ncol: u32,
}

impl From<SmallVec<[PartitionDefnData; PARTITION_SMALL_VEC_SIZE]>> for Partitions {
    fn from(nondefaults: SmallVec<[PartitionDefnData; PARTITION_SMALL_VEC_SIZE]>) -> Self {
        let nodefault_ncol_total: u32 = nondefaults.iter().map(|p| p.ncol).sum();
        let default_partition_ncol: u32 = NCOL_TOTAL - nodefault_ncol_total;
        Self {
            nondefaults,
            default_partition_ncol,
        }
    }
}

impl Partitions {
    pub fn add_partition(&mut self, idx: usize, new_partition: PartitionDefnData) {
        self.default_partition_ncol -= new_partition.ncol;
        self.nondefaults.insert(idx, new_partition)
    }

    pub fn partition_idx(&self, label: Label) -> usize {
        self.opt_label_idx(label).unwrap_or(self.nondefaults.len())
    }

    pub fn opt_label_idx(&self, label: Label) -> Option<usize> {
        self.nondefaults
            .iter()
            .position(|partition| partition.contains(label))
    }

    pub fn init_partition_values<T>(&self) -> Vec<T>
    where
        T: Default,
    {
        (0..self.total_len())
            .into_iter()
            .map(|_| Default::default())
            .collect()
    }

    pub fn total_len(&self) -> usize {
        self.nondefaults.len() + 1
    }
}

// impl std::ops::Deref for Partitions {
//     type Target = [PartitionDefnData];

//     fn deref(&self) -> &Self::Target {
//         &self.nondefaults
//     }
// }
