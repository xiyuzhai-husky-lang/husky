use std::marker::PhantomData;

use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

use crate::{id::TraceId, IsTraceProtocol};

#[derive(Debug, Serialize, Deserialize)]
pub enum TraceViewAction<TraceProtocol: IsTraceProtocol> {
    ToggleExpansion {
        trace_id: TraceId,
    },
    Marker {
        _marker: PhantomData<TraceProtocol>,
    },
    ToggleAssociatedTrace {
        trace_id: TraceId,
        associated_trace_id: TraceId,
    },
    FollowTrace {
        trace_id: TraceId,
    },
    ToggleAccompany {
        trace_id: TraceId,
    },
    SetPedestal {
        pedestal: TraceProtocol::Pedestal,
    },
}

pub struct TraceViewActionBuffer<TraceProtocol: IsTraceProtocol> {
    actions: SmallVec<[TraceViewAction<TraceProtocol>; 2]>,
}

impl<TraceProtocol: IsTraceProtocol> Default for TraceViewActionBuffer<TraceProtocol> {
    fn default() -> Self {
        Self {
            actions: Default::default(),
        }
    }
}

impl<TraceProtocol: IsTraceProtocol> TraceViewActionBuffer<TraceProtocol> {
    pub fn push(&mut self, action: TraceViewAction<TraceProtocol>) {
        self.actions.push(action)
    }

    pub fn take_actions(&mut self) -> SmallVec<[TraceViewAction<TraceProtocol>; 2]> {
        std::mem::take(&mut self.actions)
    }
}
