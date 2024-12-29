use crate::*;
use visored_mir_expr::expr::VdMirExprIdx;

type SuperVarsContext = ();
type Term = ();

pub struct VdTacticsEvaluationSession<'db> {
    db: &'db EternerDb,
    expr_terms: DashNote<(VdMirExprIdx, SuperVarsContext), Term>,
}

impl<'db> VdTacticsEvaluationSession<'db> {
    pub fn new(db: &'db EternerDb) -> Self {
        Self {
            db,
            expr_terms: DashNote::default(),
        }
    }
}
