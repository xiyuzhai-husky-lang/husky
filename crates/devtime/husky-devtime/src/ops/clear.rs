use crate::*;

impl HuskyDevtime {
    // ad hoc, use HuskyDevtimeClearM
    pub(super) fn clear(&mut self) -> HuskyDevtimeUpdateM<()> {
        // replace this with diff, try to make the trace tree look the same across code change
        self.state.clear()?;
        HuskyDevtimeUpdateM::Ok(())
    }
}
