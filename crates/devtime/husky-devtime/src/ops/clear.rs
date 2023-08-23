use crate::*;

impl Devtime {
    // ad hoc, use DevtimeClearM
    pub(super) fn clear(&mut self) -> DevtimeOldState {
        self.state.clear_pop()
    }
}
