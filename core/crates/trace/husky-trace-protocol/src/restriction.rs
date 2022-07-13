mod label;
mod partition;

pub use label::*;
pub use partition::*;
use sycamore::prelude::Signalable;
use vec_like::VecSet;

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Restriction {
    is_specific: bool,
    specific_sample_id: SampleId,
    partitions: Vec<PartitionDefnData>,
    arrivals: VecSet<TraceId>,
    enters: VecSet<TraceId>,
}

impl Restriction {
    pub fn is_specific(&self) -> bool {
        self.is_specific
    }
    pub fn is_generic(&self) -> bool {
        !self.is_specific
    }

    pub fn partitions(&self) -> &Vec<PartitionDefnData> {
        &self.partitions
    }

    pub fn arrivals(&self) -> &VecSet<TraceId> {
        &self.arrivals
    }

    pub fn enters(&self) -> &VecSet<TraceId> {
        &self.enters
    }

    pub fn opt_sample_id(&self) -> Option<SampleId> {
        if self.is_specific {
            Some(self.specific_sample_id)
        } else {
            None
        }
    }

    pub fn new_specific(specific_sample_id: SampleId) -> Restriction {
        Self {
            is_specific: true,
            specific_sample_id,
            partitions: vec![PartitionDefnData {
                ncol: 7,
                variant: PartitionDefnDataVariant::Other,
            }],
            arrivals: Default::default(),
            enters: Default::default(),
        }
    }

    pub fn arrival(&self, trace_id: TraceId) -> bool {
        self.arrivals.has(trace_id)
    }

    pub fn set_sample_id(&mut self, sample_id: SampleId) {
        self.specific_sample_id = sample_id
    }

    pub fn toggle_arrival(&mut self, trace_id: TraceId) {
        self.arrivals.toggle(trace_id)
    }

    pub fn toggle_enter(&mut self, trace_id: TraceId) {
        self.enters.toggle(trace_id)
    }

    pub fn toggle_is_specific(&mut self) {
        self.is_specific = !self.is_specific;
    }

    pub fn add_partition(&mut self, idx: usize, new_partition: PartitionDefnData) {
        self.partitions.last_mut().unwrap().ncol -= new_partition.ncol;
        self.partitions.insert(idx, new_partition)
    }
}

impl Signalable for Restriction {}

impl Default for Restriction {
    fn default() -> Self {
        Self {
            is_specific: true,
            specific_sample_id: SampleId(0),
            partitions: vec![PartitionDefnData {
                ncol: 7,
                variant: PartitionDefnDataVariant::Other,
            }],
            arrivals: Default::default(),
            enters: Default::default(),
        }
    }
}
