use crate::*;
use visored_mir_expr::expr::VdMirExprIdx;

type SuperVarsContext = ();
type Term = ();

pub struct VdTacticsEvaluationSession<'db> {
    db: &'db EternerDb,
}

impl<'db> VdTacticsEvaluationSession<'db> {
    pub fn new(db: &'db EternerDb) -> Self {
        Self { db }
    }
}
