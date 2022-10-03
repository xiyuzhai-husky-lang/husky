mod partition;

use husky_feature_protocol::FeatureId;
use husky_signal::Signalable;
pub use partition::*;
use vec_like::VecPairMap;

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Presentation {
    kind: PresentationKind,
    sample_id: SampleId,
    restriction: Restriction,
    partitions: Partitions, // don't need this when we have monad
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PresentationKind {
    Generic,
    Specific,
    Panic,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Restriction {
    None,
    Arrival {
        trace_id: TraceId,
        feature_id: FeatureId,
        arrival_restriction_kind: ArrivalRestrictionKind,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArrivalRestrictionKind {
    Default,
    Return,
    DeprecatedStrikeEvil, // deprecated
}

impl Default for Restriction {
    fn default() -> Self {
        Restriction::None
    }
}

impl Restriction {
    pub fn clear(&mut self) {
        *self = Restriction::None
    }
}

impl Presentation {
    pub fn clear(&mut self) {
        self.restriction.clear()
    }

    pub fn restriction(&self) -> Restriction {
        self.restriction
    }

    pub fn activate_trace(&self, trace_data: &TraceData) -> Presentation {
        let presentation = self.clone();
        todo!()
    }

    pub fn is_specific(&self) -> bool {
        self.kind == PresentationKind::Specific
    }

    pub fn is_generic(&self) -> bool {
        matches!(self.kind, PresentationKind::Generic)
    }

    pub fn partitions(&self) -> &Partitions {
        &self.partitions
    }

    pub fn opt_sample_id(&self) -> Option<SampleId> {
        match self.kind {
            PresentationKind::Generic => None,
            PresentationKind::Specific => Some(self.sample_id),
            PresentationKind::Panic => Some(self.sample_id),
        }
    }

    pub fn sample_id(&self) -> SampleId {
        self.sample_id
    }

    pub fn new_specific(specific_sample_id: SampleId) -> Presentation {
        Self {
            kind: PresentationKind::Specific,
            sample_id: specific_sample_id,
            partitions: Default::default(),
            restriction: Default::default(),
        }
    }

    pub fn set_sample_id(&mut self, sample_id: SampleId) {
        self.sample_id = sample_id
    }

    pub fn toggle_kind(&mut self) {
        self.kind = match self.kind {
            PresentationKind::Generic => PresentationKind::Specific,
            PresentationKind::Specific => PresentationKind::Generic,
            PresentationKind::Panic => unreachable!(),
        }
    }

    pub fn set_specific(&mut self, sample_id: SampleId) {
        self.kind = PresentationKind::Specific;
        self.sample_id = sample_id;
    }

    pub fn add_partition(&mut self, idx: usize, new_partition: PartitionDefnData) {
        self.partitions.add_partition(idx, new_partition)
    }
}

impl Signalable for Presentation {}

impl Default for Presentation {
    fn default() -> Self {
        Self {
            kind: PresentationKind::Generic,
            sample_id: SampleId(0),
            partitions: Default::default(),
            restriction: Default::default(),
        }
    }
}
