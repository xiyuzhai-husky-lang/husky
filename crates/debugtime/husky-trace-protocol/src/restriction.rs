mod partition;

use husky_signal::Signalable;
pub use partition::*;
use vec_like::VecPairMap;

use super::*;
use serde::{Deserialize, Serialize};

pub type PinnedArrivals = VecPairMap<TraceId, ArrivalRefinedControl>;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Restriction {
    kind: RestrictionKind,
    sample_id: SampleId,
    partitions: Partitions,
    pinned_arrivals: PinnedArrivals,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RestrictionKind {
    Generic,
    Specific,
    Panic,
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
    pub fn clear(&mut self) {
        self.pinned_arrivals.clear()
    }

    pub fn toggle_pin(&self, trace: &TraceData) -> Self {
        let mut restriction = self.clone();
        if !trace.always_arrived() {
            restriction.toggle_arrival(trace.id)
        }
        restriction
    }

    pub fn is_specific(&self) -> bool {
        self.kind == RestrictionKind::Specific
    }

    pub fn is_generic(&self) -> bool {
        self.kind == RestrictionKind::Generic
    }

    pub fn partitions(&self) -> &Partitions {
        &self.partitions
    }

    pub fn arrivals(&self) -> &PinnedArrivals {
        &self.pinned_arrivals
    }

    pub fn opt_sample_id(&self) -> Option<SampleId> {
        match self.kind {
            RestrictionKind::Generic => None,
            RestrictionKind::Specific => Some(self.sample_id),
            RestrictionKind::Panic => Some(self.sample_id),
        }
    }

    pub fn sample_id(&self) -> SampleId {
        self.sample_id
    }

    pub fn new_specific(specific_sample_id: SampleId) -> Restriction {
        Self {
            kind: RestrictionKind::Specific,
            sample_id: specific_sample_id,
            partitions: Default::default(),
            pinned_arrivals: Default::default(),
        }
    }

    pub fn arrival_refined_control(&self, trace_id: TraceId) -> Option<&ArrivalRefinedControl> {
        self.pinned_arrivals.get_entry(trace_id).map(|p| &p.1)
    }

    pub fn set_sample_id(&mut self, sample_id: SampleId) {
        self.sample_id = sample_id
    }

    pub(crate) fn toggle_arrival(&mut self, trace_id: TraceId) {
        self.pinned_arrivals.toggle(trace_id)
    }

    pub fn toggle_arrival_refined_strike_evil(&mut self, trace_id: TraceId) {
        self.pinned_arrivals[trace_id].1.toggle_strike_evil()
    }

    pub fn toggle_kind(&mut self) {
        self.kind = match self.kind {
            RestrictionKind::Generic => RestrictionKind::Specific,
            RestrictionKind::Specific => RestrictionKind::Generic,
            RestrictionKind::Panic => unreachable!(),
        }
    }

    pub fn set_specific(&mut self, sample_id: SampleId) {
        self.kind = RestrictionKind::Specific;
        self.sample_id = sample_id;
    }

    pub fn add_partition(&mut self, idx: usize, new_partition: PartitionDefnData) {
        self.partitions.add_partition(idx, new_partition)
    }
}

impl Signalable for Restriction {}

impl Default for Restriction {
    fn default() -> Self {
        Self {
            kind: RestrictionKind::Generic,
            sample_id: SampleId(0),
            partitions: Default::default(),
            pinned_arrivals: Default::default(),
        }
    }
}
