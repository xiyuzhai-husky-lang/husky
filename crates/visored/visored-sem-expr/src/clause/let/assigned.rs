use super::*;
use pattern::VdSemPattern;
use visored_syn_expr::clause::r#let::assigned::VdSynLetAssignedResolution;

#[derive(Debug, PartialEq, Eq)]
pub struct VdSemLetAssignedDispatch {
    pattern: VdSemPattern,
    assignment: VdSemExprIdx,
}

impl ToVdSem<VdSemLetAssignedDispatch> for &VdSynLetAssignedResolution {
    fn to_vd_sem(self, builder: &mut VdSemExprBuilder) -> VdSemLetAssignedDispatch {
        VdSemLetAssignedDispatch {
            pattern: todo!(),
            assignment: todo!(),
        }
    }
}
