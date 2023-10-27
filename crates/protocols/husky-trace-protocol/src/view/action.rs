use std::marker::PhantomData;

use smallvec::SmallVec;

use crate::{cache::TraceCacheAction, id::TraceId};

pub enum TraceViewAction<VisualComponent> {
    ToggleExpansion {
        trace_id: TraceId,
    },
    Marker {
        _marker: PhantomData<VisualComponent>,
    },
}

impl<VisualComponent> TraceViewAction<VisualComponent> {
    fn try_resolve_at_client_side(&self) -> Option<TraceCacheAction<VisualComponent>> {
        todo!()
    }

    fn resolve_at_server_side(&self) -> SmallVec<[TraceCacheAction<VisualComponent>; 3]> {
        todo!()
    }
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
