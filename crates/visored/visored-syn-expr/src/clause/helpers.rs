use super::*;

pub enum VdSynClauseChild {
    Expr(VdSynExprIdx),
}

impl VdSynClauseData {
    pub(crate) fn children(&self) -> Vec<VdSynClauseChild> {
        match *self {
            VdSynClauseData::Let { formula, .. } => vec![VdSynClauseChild::Expr(formula)],
            VdSynClauseData::Assume { formula, .. } => vec![VdSynClauseChild::Expr(formula)],
            VdSynClauseData::Then { formula, .. } => vec![VdSynClauseChild::Expr(formula)],
            VdSynClauseData::Todo(..) => vec![],
        }
    }
}
