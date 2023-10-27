use smallvec::SmallVec;

use crate::id::TraceId;

pub enum TraceViewAction {
    ToggleExpansion { trace_id: TraceId },
}

#[derive(Default)]
pub struct TraceViewActionBuffer {
    actions: SmallVec<[TraceViewAction; 2]>,
}

impl TraceViewActionBuffer {
    pub fn push(&mut self, action: TraceViewAction) {
        self.actions.push(action)
    }

    pub fn take_actions(&mut self) -> SmallVec<[TraceViewAction; 2]> {
        std::mem::take(&mut self.actions)
    }
}
