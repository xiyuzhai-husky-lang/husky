use crate::*;

impl HuskyDebugtime {
    pub(super) fn clear(&mut self) -> HuskyDebugtimeMakeChangeM<()> {
        // replace this with diff, try to make the trace tree look the same across code change
        self.state = Default::default();
        HuskyDebugtimeMakeChangeM::Ok(())
    }
}
