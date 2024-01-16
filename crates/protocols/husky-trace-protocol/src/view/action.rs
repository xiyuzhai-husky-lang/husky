use std::marker::PhantomData;

use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

use crate::id::TraceId;

#[derive(Debug, Serialize, Deserialize)]
pub enum TraceViewAction<TraceProtocol> {
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
    FocusTrace {
        trace_id: TraceId,
    },
}

pub struct TraceViewActionBuffer<TraceProtocol> {
    actions: SmallVec<[TraceViewAction<TraceProtocol>; 2]>,
}

impl<TraceProtocol> Default for TraceViewActionBuffer<TraceProtocol> {
    fn default() -> Self {
        Self {
            actions: Default::default(),
        }
    }
}

impl<TraceProtocol> TraceViewActionBuffer<TraceProtocol> {
    pub fn push(&mut self, action: TraceViewAction<TraceProtocol>) {
        self.actions.push(action)
    }

    pub fn take_actions(&mut self) -> SmallVec<[TraceViewAction<TraceProtocol>; 2]> {
        std::mem::take(&mut self.actions)
    }
}
