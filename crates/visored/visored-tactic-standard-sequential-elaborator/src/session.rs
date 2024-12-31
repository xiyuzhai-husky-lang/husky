use crate::*;
use visored_mir_expr::expr::VdMirExprIdx;

type SuperVarsContext = ();
type Term = ();

pub struct VdMirStmtElaborationSession<'db> {
    db: &'db EternerDb,
}

impl<'db> VdMirStmtElaborationSession<'db> {
    pub fn new(db: &'db EternerDb) -> Self {
        Self { db }
    }
}
