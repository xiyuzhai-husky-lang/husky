use crate::*;
use visored_mir_expr::expr::VdMirExprIdx;

type SuperVarsContext = ();
type Term = ();

pub struct VdMirSession<'db> {
    db: &'db EternerDb,
}

impl<'db> VdMirSession<'db> {
    pub fn new(db: &'db EternerDb) -> Self {
        Self { db }
    }
}
