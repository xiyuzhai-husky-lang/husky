mod partition;

pub use partition::*;

use super::*;
use serde::{Deserialize, Serialize};
use vec_like::VecSet;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Presentation {
    kind: PresentationKind,
    sample_id: SampleId,
    opt_active_trace_id: Option<TraceId>,
    restriction: Option<Restriction>,
    pins: VecSet<TraceId>,
    partitions: Partitions, // don't need this when we have monad
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PresentationKind {
    Generic,
    Specific,
    Panic,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Restriction {
    pub trace_id: TraceId,
    pub kind: RestrictionKind,
}

impl Restriction {
    pub fn mimic<'a>(self, f: &'a impl Fn(TraceId) -> Option<&'a TraceData>) -> Option<Self> {
        let trace_data = f(self.trace_id)?;
        let trace_id = trace_data.id;
        Some(Restriction {
            trace_id,
            kind: self.kind,
        })
    }
}

impl std::fmt::Debug for Restriction {
    fn fmt(&self, f: &mut __private::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Arrival({};{:?})", self.trace_id, self.kind)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RestrictionKind {
    Arrival,
    Return,
}

impl Presentation {
    pub fn mimic<'a>(&self, f: &'a impl Fn(TraceId) -> Option<&'a TraceData>) -> Self {
        Self {
            kind: self.kind,
            sample_id: self.sample_id,
            opt_active_trace_id: self.opt_active_trace_id.map(f).flatten().map(|n| n.id),
            restriction: self.restriction.map(|res| res.mimic(f)).flatten(),
            pins: self
                .pins
                .iter()
                .filter_map(|pin| f(*pin))
                .map(|n| n.id)
                .collect(),
            partitions: self.partitions.clone(),
        }
    }

    pub fn clear(&mut self) {
        self.restriction = None;
        self.opt_active_trace_id = None;
        self.pins.clear()
    }

    pub fn kind(&self) -> PresentationKind {
        self.kind
    }

    pub fn restriction(&self) -> Option<Restriction> {
        self.restriction
    }

    pub fn opt_active_trace_id(&self) -> Option<TraceId> {
        self.opt_active_trace_id
    }

    pub fn activate_trace(&mut self, trace_data: &TraceData) {
        self.opt_active_trace_id = Some(trace_data.id);
        self.restriction = Some(Restriction {
            trace_id: trace_data.id,
            kind: RestrictionKind::Arrival,
        })
    }

    pub fn activate_trace_out_of_place(&self, trace_data: &TraceData) -> Presentation {
        let mut presentation = self.clone();
        presentation.activate_trace(trace_data);
        presentation
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
            opt_active_trace_id: todo!(),
            pins: todo!(),
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

    pub fn add_partition(&mut self, idx: usize, new_partition: Partition) {
        self.partitions.add_partition(idx, new_partition)
    }

    pub fn shrink_partition(&mut self, idx: usize) {
        self.partitions.shrink_partition(idx)
    }

    pub fn expand_partition(&mut self, idx: usize) {
        self.partitions.expand_partition(idx)
    }

    pub fn remove_partition(&mut self, idx: usize) {
        self.partitions.remove_partition(idx)
    }

    pub fn restrict_to_arrival(&mut self, _trace_id: TraceId) {
        match self.restriction {
            Some(ref mut res) => res.kind = RestrictionKind::Arrival,
            None => (),
        }
    }

    pub fn restrict_to_return(&mut self, _trace_id: TraceId) {
        match self.restriction {
            Some(ref mut res) => res.kind = RestrictionKind::Return,
            None => (),
        }
    }

    pub fn is_pinned(&self, id: TraceId) -> bool {
        self.pins.contains(&id)
    }

    pub fn toggle_pin(&mut self, id: TraceId) {
        self.pins.toggle(id)
    }

    pub fn pins(&self) -> &[TraceId] {
        &self.pins
    }
}

impl Default for Presentation {
    fn default() -> Self {
        Self {
            kind: PresentationKind::Generic,
            sample_id: SampleId(0),
            partitions: Default::default(),
            restriction: Default::default(),
            opt_active_trace_id: None,
            pins: Default::default(),
        }
    }
}
