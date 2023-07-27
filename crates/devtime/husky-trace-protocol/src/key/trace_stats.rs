use smallvec::SmallVec;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Hash)]
pub struct TraceStatsKey {
    pub trace_id: TraceId,
    pub partitions: Partitions,
}

const PARTITION_SMALL_VEC_SIZE: usize = 4;
const NCOL_TOTAL: u32 = 7;

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Hash)]
pub struct Partitions {
    partitions: SmallVec<[Partition; PARTITION_SMALL_VEC_SIZE]>,
    others_ncol: u32,
}

impl std::fmt::Debug for Partitions {
    fn fmt(&self, f: &mut __private::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Partitions({:?};{})",
            &self.partitions, &self.others_ncol
        ))
    }
}

impl Default for Partitions {
    fn default() -> Self {
        Self {
            partitions: Default::default(),
            others_ncol: NCOL_TOTAL,
        }
    }
}

#[test]
fn test_partition_idx() {
    let mut partitions = Partitions::default();
    assert_eq!(partitions.opt_nondefault_partition_idx(Label(0)), None);
    assert_eq!(partitions.partition_idx(Label(0)), 0);
    assert_eq!(partitions.opt_nondefault_partition_idx(Label(1)), None);
    assert_eq!(partitions.partition_idx(Label(1)), 0);
    assert_eq!(partitions.opt_nondefault_partition_idx(Label(5)), None);
    assert_eq!(partitions.partition_idx(Label(5)), 0);
    assert!(!partitions.is_nondefault(Label(0)));
    assert!(!partitions.is_nondefault(Label(5)));
    partitions.add_partition(
        0,
        Partition {
            ncol: 3,
            variant: PartitionVariant::Label(Label(0)),
        },
    );
    assert_eq!(partitions.opt_nondefault_partition_idx(Label(0)), Some(0));
    assert_eq!(partitions.partition_idx(Label(0)), 0);
    assert_eq!(partitions.opt_nondefault_partition_idx(Label(1)), None);
    assert_eq!(partitions.partition_idx(Label(1)), 1);
    assert_eq!(partitions.opt_nondefault_partition_idx(Label(5)), None);
    assert_eq!(partitions.partition_idx(Label(5)), 1);
    assert!(partitions.is_nondefault(Label(0)));
    assert!(!partitions.is_nondefault(Label(5)));
}

impl From<SmallVec<[Partition; PARTITION_SMALL_VEC_SIZE]>> for Partitions {
    fn from(nondefaults: SmallVec<[Partition; PARTITION_SMALL_VEC_SIZE]>) -> Self {
        let nodefault_ncol_total: u32 = nondefaults
            .iter()
            .map(|p| {
                assert!(p.ncol > 0);
                p.ncol
            })
            .sum();
        let default_partition_ncol: u32 = NCOL_TOTAL - nodefault_ncol_total;
        assert!(default_partition_ncol > 0);
        Self {
            partitions: nondefaults,
            others_ncol: default_partition_ncol,
        }
    }
}

impl Partitions {
    pub fn add_partition(&mut self, idx: usize, new_partition: Partition) {
        assert_ne!(new_partition.variant, PartitionVariant::Default);
        self.others_ncol -= new_partition.ncol;
        assert!(self.others_ncol > 0);
        assert!(new_partition.ncol > 0);
        self.partitions.insert(idx, new_partition)
    }

    pub fn expand_partition(&mut self, index: usize) {
        assert!(self.others_ncol > 0);
        self.partitions[index].ncol += 1;
        self.others_ncol -= 1;
    }

    pub fn shrink_partition(&mut self, index: usize) {
        assert!(self.partitions[index].ncol > 1);
        self.partitions[index].ncol -= 1;
        self.others_ncol += 1;
    }

    pub fn remove_partition(&mut self, index: usize) {
        assert!(index < self.partitions.len());
        let removed: Partition = self.partitions.remove(index);
        self.others_ncol += removed.ncol;
    }

    pub fn partition_idx(&self, label: Label) -> usize {
        self.opt_nondefault_partition_idx(label)
            .unwrap_or(self.partitions.len())
    }
    pub fn partition_ncol(&self, partition_idx: usize) -> u32 {
        self.partitions
            .get(partition_idx)
            .map(|d| d.ncol)
            .unwrap_or(self.others_ncol)
    }

    pub fn opt_nondefault_partition_idx(&self, label: Label) -> Option<usize> {
        self.partitions
            .iter()
            .position(|partition| partition.contains(label))
    }

    pub fn is_nondefault(&self, label: Label) -> bool {
        self.opt_nondefault_partition_idx(label).is_some()
    }

    pub fn init_partition_values<T>(&self) -> Vec<(Partition, T)>
    where
        T: Default,
    {
        (0..self.total_len())
            .into_iter()
            .map(|i| (self.defn_data(i), Default::default()))
            .collect()
    }

    fn defn_data(&self, i: usize) -> Partition {
        if i < self.partitions.len() {
            self.partitions[i].clone()
        } else {
            self.default_partition_defn_data()
        }
    }

    fn default_partition_defn_data(&self) -> Partition {
        Partition {
            ncol: self.others_ncol,
            variant: PartitionVariant::Default,
        }
    }

    pub fn total_len(&self) -> usize {
        self.partitions.len() + 1
    }
}

// impl std::ops::Deref for Partitions {
//     type Target = [Partition];

//     fn deref(&self) -> &Self::Target {
//         &self.nondefaults
//     }
// }
