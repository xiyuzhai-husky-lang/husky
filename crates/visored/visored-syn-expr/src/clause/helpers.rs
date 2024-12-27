use super::*;

pub enum VdSynClauseChild {
    Expr(VdSynExprIdx),
}

impl VdSynClauseData {
    pub(crate) fn children(&self) -> Vec<VdSynClauseChild> {
        match *self {
            VdSynClauseData::Let { formula, .. } => vec![VdSynClauseChild::Expr(formula)],
            VdSynClauseData::Assume { formula, .. } => vec![VdSynClauseChild::Expr(formula)],
            VdSynClauseData::Goal { formula, .. } => vec![VdSynClauseChild::Expr(formula)],
            VdSynClauseData::Have { formula, .. } => vec![VdSynClauseChild::Expr(formula)],
            VdSynClauseData::Show {
                left_math_delimiter_token_idx: left_dollar_token_idx,
                formula,
                right_math_delimiter_token_idx: right_dollar_token_idx,
            } => todo!(),
        }
    }
}
