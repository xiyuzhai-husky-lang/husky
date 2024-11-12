use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct VdSynLetStmtAssignedResolution {
    pattern: VdSynExprIdx,
    assignment: VdSynExprIdx,
}

impl<'db> VdSynExprBuilder<'db> {
    pub fn build_let_assigned_resolution(
        &self,
        pattern: VdSynExprIdx,
        assignment: VdSynExprIdx,
    ) -> VdSynLetStmtAssignedResolution {
        VdSynLetStmtAssignedResolution {
            pattern,
            assignment,
        }
    }
}
