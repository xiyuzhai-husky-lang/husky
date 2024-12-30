use crate::*;
use visored_mir_expr::expr::VdMirExprIdx;

type SuperVarsContext = ();
type Term = ();

pub struct VdMirTacticEvaluationSession<'db> {
    db: &'db EternerDb,
}

impl<'db> VdMirTacticEvaluationSession<'db> {
    pub fn new(db: &'db EternerDb) -> Self {
        Self { db }
    }
}
