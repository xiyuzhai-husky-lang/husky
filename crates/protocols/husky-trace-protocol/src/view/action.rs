use std::marker::PhantomData;

use husky_visual_protocol::IsVisualComponent;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

use crate::{
    cache::{action::TraceCacheActionToggleExpansion, TraceCache, TraceCacheAction},
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
}

impl<VisualComponent> TraceViewAction<VisualComponent>
where
    VisualComponent: IsVisualComponent,
{
    pub fn try_resolve_at_client_side(
        &self,
        cache: &TraceCache<VisualComponent>,
    ) -> Option<TraceCacheAction<VisualComponent>> {
        match self {
            &TraceViewAction::ToggleExpansion { trace_id } => {
                let trace_cache_entry = &cache[trace_id];
                if !trace_cache_entry.expanded() {
                    trace_cache_entry.subtraces()?;
                }
                Some(TraceCacheActionToggleExpansion::new(trace_id).into())
            }
            TraceViewAction::Marker { _marker } => todo!(),
        }
    }

    pub fn resolve_at_server_side(&self) -> SmallVec<[TraceCacheAction<VisualComponent>; 3]> {
        match self {
            TraceViewAction::ToggleExpansion { trace_id } => todo!(),
            TraceViewAction::Marker { _marker } => todo!(),
        }
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
