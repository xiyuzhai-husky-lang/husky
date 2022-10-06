use crate::*;

impl HuskyDevtime {
    // ad hoc, use HuskyDevtimeClearM
    pub(super) fn clear(&mut self) -> HuskyDevtimeUpdateM<HuskyDevtimeOldState> {
        HuskyDevtimeUpdateM::Ok(self.state.clear_pop()?)
    }
}
