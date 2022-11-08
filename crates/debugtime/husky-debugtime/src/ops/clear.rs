use crate::*;

impl Debugtime {
    // ad hoc, use DebugtimeClearM
    pub(super) fn clear(&mut self) -> DebugtimeUpdateM<DebugtimeOldState> {
        DebugtimeUpdateM::Ok(self.state.clear_pop()?)
    }
}
