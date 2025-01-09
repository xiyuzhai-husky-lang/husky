pub mod assigned;
pub mod placeholder;

use self::{assigned::VdSemLetAssignedDispatch, placeholder::VdSemLetPlaceholderDispatch};
use super::*;
use visored_syn_expr::clause::r#let::VdSynLetClauseResolution;

#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq)]
pub enum VdSemLetClauseDispatch {
    Assigned(VdSemLetAssignedDispatch),
    Placeholder(VdSemLetPlaceholderDispatch),
}

impl<'a> VdSemExprBuilder<'a> {
    pub(super) fn build_let_clause(
        &mut self,
        left_dollar_token_idx: LxRoseTokenIdx,
        formula: VdSynExprIdx,
        right_dollar_token_idx: LxRoseTokenIdx,
        resolution: &VdSynLetClauseResolution,
    ) -> VdSemClauseData {
        let dispatch = resolution.to_vd_sem(self);
        VdSemClauseData::Let {
            left_math_delimiter_token_idx: left_dollar_token_idx,
            formula: (formula, self.ty_menu().prop).to_vd_sem(self),
            right_math_delimiter_token_idx: right_dollar_token_idx,
            dispatch,
        }
    }
}

impl ToVdSem<VdSemLetClauseDispatch> for &VdSynLetClauseResolution {
    fn to_vd_sem(self, builder: &mut VdSemExprBuilder) -> VdSemLetClauseDispatch {
        match self {
            VdSynLetClauseResolution::Assigned(slf) => slf.to_vd_sem(builder).into(),
            VdSynLetClauseResolution::Placeholder(slf) => slf.to_vd_sem(builder).into(),
        }
    }
}
