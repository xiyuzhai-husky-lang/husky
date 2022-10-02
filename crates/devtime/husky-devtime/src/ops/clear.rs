use crate::*;

impl HuskyDevtime {
    // ad hoc, use HuskyDebugtimeClearM
    pub(super) fn clear(&mut self) -> HuskyDebugtimeUpdateM<()> {
        // replace this with diff, try to make the trace tree look the same across code change
        self.state.clear()?;
        HuskyDebugtimeUpdateM::Ok(())
    }
}
