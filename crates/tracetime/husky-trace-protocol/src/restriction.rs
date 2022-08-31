mod partition;

use husky_signal::Signalable;
pub use partition::*;
use vec_like::VecPairMap;

use super::*;
use serde::{Deserialize, Serialize};

pub type Arrivals = VecPairMap<TraceId, ArrivalRefinedControl>;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Restriction {
    is_specific: bool,
    sample_id: SampleId,
    partitions: Partitions,
    arrivals: Arrivals,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct ArrivalRefinedControl {
    strike_evil: bool,
}

impl ArrivalRefinedControl {
    pub fn strike_evil(&self) -> bool {
        self.strike_evil
    }

    pub fn toggle_strike_evil(&mut self) {
        self.strike_evil = !self.strike_evil
    }
}

impl Signalable for ArrivalRefinedControl {}

impl Restriction {
    pub fn is_specific(&self) -> bool {
        self.is_specific
    }
    pub fn is_generic(&self) -> bool {
        !self.is_specific
    }

    pub fn partitions(&self) -> &Partitions {
        &self.partitions
    }

    pub fn arrivals(&self) -> &Arrivals {
        &self.arrivals
    }

    pub fn opt_sample_id(&self) -> Option<SampleId> {
        if self.is_specific {
            Some(self.sample_id)
        } else {
            None
        }
    }

    pub fn sample_id(&self) -> SampleId {
        self.sample_id
    }

    pub fn new_specific(specific_sample_id: SampleId) -> Restriction {
        Self {
            is_specific: true,
            sample_id: specific_sample_id,
            partitions: Default::default(),
            arrivals: Default::default(),
        }
    }

    pub fn arrival_refined_control(&self, trace_id: TraceId) -> Option<&ArrivalRefinedControl> {
        self.arrivals.get_entry(trace_id).map(|p| &p.1)
    }

    pub fn set_sample_id(&mut self, sample_id: SampleId) {
        self.sample_id = sample_id
    }

    pub fn toggle_arrival(&mut self, trace_id: TraceId) {
        self.arrivals.toggle(trace_id)
    }

    pub fn toggle_arrival_refined_strike_evil(&mut self, trace_id: TraceId) {
        self.arrivals[trace_id].1.toggle_strike_evil()
    }

    pub fn toggle_is_specific(&mut self) {
        self.is_specific = !self.is_specific;
    }

    pub fn add_partition(&mut self, idx: usize, new_partition: PartitionDefnData) {
        self.partitions.add_partition(idx, new_partition)
    }
}

impl Signalable for Restriction {}

impl Default for Restriction {
    fn default() -> Self {
        Self {
            is_specific: false,
            sample_id: SampleId(0),
            partitions: Default::default(),
            arrivals: Default::default(),
        }
    }
}
