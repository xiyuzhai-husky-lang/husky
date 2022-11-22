use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Partition {
    pub ncol: u32,
    pub variant: PartitionVariant,
}

impl std::fmt::Display for Partition {
    fn fmt(&self, f: &mut __private::Formatter<'_>) -> std::fmt::Result {
        match self.variant {
            PartitionVariant::Label(label) => label.0.fmt(f),
            PartitionVariant::LabelSet(_) => todo!(),
            PartitionVariant::Default => "default".fmt(f),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum PartitionVariant {
    Label(Label),
    LabelSet(Vec<Label>),
    Default,
}

#[test]
fn test_contains() {
    let partition = Partition {
        ncol: 3,
        variant: PartitionVariant::Label(Label(3)),
    };
    assert!(partition.contains(Label(3)));
    assert!(!partition.contains(Label(6)));
    assert!(!partition.contains(Label(0)));
}

impl Partition {
    pub fn contains(&self, target: Label) -> bool {
        match self.variant {
            PartitionVariant::Label(label) => label == target,
            PartitionVariant::LabelSet(ref labels) => labels.contains(&target),
            PartitionVariant::Default => panic!(),
        }
    }

    pub fn name(&self) -> String {
        match self.variant {
            PartitionVariant::Label(l) => format!("{}", l.0),
            PartitionVariant::LabelSet(_) => todo!(),
            PartitionVariant::Default => "_".to_string(),
        }
    }
}

pub struct PartitionedSampler<'a, T> {
    // suppose there are three partitions (including Other)
    // then partition_filled is 0...0111
    // the first partition is filled iff the last digit of partition_filled is 0
    // the second partition is filled iff the last second digit of partition_filled is 0
    flags: u32,
    partitions: &'a Partitions,
    partitioned_samples: Vec<(Partition, Vec<(SampleId, T)>)>,
    col_len: u32,
}

impl<'a, T> PartitionedSampler<'a, T> {
    pub fn new(partitions: &'a Partitions) -> Self {
        let flags: u32 = (!0u32 << partitions.total_len()) ^ (!0u32);
        Self {
            flags,
            partitions,
            partitioned_samples: partitions.init_partition_values(),
            col_len: 5,
        }
    }

    // returns a flag indicating whether the partitions are all full
    pub fn process<E>(
        &mut self,
        labeled_data: &LabeledData,
        f: impl FnOnce() -> Result<T, E>,
    ) -> Result<bool, (SampleId, E)> {
        let i = self.partitions.partition_idx(labeled_data.label);
        let max_samples_len = (self.partition_ncol(i) * self.col_len) as usize;
        assert!(max_samples_len > 0);
        let samples = &mut self.partitioned_samples[i].1;
        if samples.len() < max_samples_len {
            samples.push((
                labeled_data.sample_id,
                f().map_err(|e| (labeled_data.sample_id, e))?,
            ));
            if samples.len() == max_samples_len {
                // unset the bit at i
                self.flags &= !(1 << i);
            }
        } else {
            assert!((self.flags & (1 << i)) == 0)
        }
        Ok(self.flags == 0)
    }

    fn partition_ncol(&self, partition_idx: usize) -> u32 {
        self.partitions.partition_ncol(partition_idx)
    }

    pub fn finish(self) -> Vec<(Partition, Vec<(SampleId, T)>)> {
        self.partitioned_samples
    }
}
