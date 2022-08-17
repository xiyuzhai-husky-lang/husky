use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct PartitionDefnData {
    pub ncol: u32,
    pub variant: PartitionDefnDataVariant,
}

impl std::fmt::Display for PartitionDefnData {
    fn fmt(&self, f: &mut __private::Formatter<'_>) -> std::fmt::Result {
        match self.variant {
            PartitionDefnDataVariant::Label(label) => label.0.fmt(f),
            PartitionDefnDataVariant::LabelSet(_) => todo!(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum PartitionDefnDataVariant {
    Label(Label),
    LabelSet(Vec<Label>),
}

impl PartitionDefnData {
    pub fn contains(&self, label: Label) -> bool {
        match self.variant {
            PartitionDefnDataVariant::Label(label) => label == label,
            PartitionDefnDataVariant::LabelSet(ref labels) => labels.contains(&label),
        }
    }

    pub fn name(&self) -> String {
        match self.variant {
            PartitionDefnDataVariant::Label(l) => format!("{}", l.0),
            PartitionDefnDataVariant::LabelSet(_) => todo!(),
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
    partitioned_samples: Vec<Vec<(SampleId, T)>>,
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
        label: Label,
        f: impl FnOnce() -> Result<(SampleId, T), E>,
    ) -> Result<bool, E> {
        let i = self.partitions.partition_idx(label);
        let max_samples_len = (self.partition_ncol(i) * self.col_len) as usize;
        let samples = &mut self.partitioned_samples[i];
        if samples.len() < max_samples_len {
            samples.push(f()?);
            if samples.len() == max_samples_len {
                self.flags &= !(1 << i);
            }
        } else {
            assert!((self.flags & (1 << i)) == 0)
        }
        Ok(self.flags == 0)
    }

    fn partition_ncol(&self, partition_idx: usize) -> u32 {
        todo!()
    }

    pub fn finish(self) -> Vec<Vec<(SampleId, T)>> {
        self.partitioned_samples
    }
}
