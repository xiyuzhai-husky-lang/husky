use std::marker::PhantomData;


use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

use crate::{
    id::TraceId,
};

#[derive(Debug, Serialize, Deserialize)]
pub enum TraceViewAction<VisualComponent> {
    ToggleExpansion {
        trace_id: TraceId,
    },
    Marker {
        _marker: PhantomData<VisualComponent>,
    },
    ToggleAssociatedTrace {
        trace_id: TraceId,
        associated_trace_id: TraceId,
    },
}

#[derive(Default)]
pub struct TraceViewActionBuffer<VisualComponent> {
    actions: SmallVec<[TraceViewAction<VisualComponent>; 2]>,
}

impl<VisualComponent> TraceViewActionBuffer<VisualComponent> {
    pub fn push(&mut self, action: TraceViewAction<VisualComponent>) {
        self.actions.push(action)
    }

    pub fn take_actions(&mut self) -> SmallVec<[TraceViewAction<VisualComponent>; 2]> {
        std::mem::take(&mut self.actions)
    }
}
