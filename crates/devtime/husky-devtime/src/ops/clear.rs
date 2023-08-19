use crate::*;

impl Devtime {
    // ad hoc, use DevtimeClearM
    pub(super) fn clear(&mut self) -> DevtimeUpdateM<DevtimeOldState> {
        DevtimeUpdateM::Ok(self.state.clear_pop()?)
    }
}
