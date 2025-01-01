use crate::*;
use floated_sequential::db::FloaterDb;
use visored_mir_expr::expr::VdMirExprIdx;

type SuperVarsContext = ();
type Term = ();

pub struct VdBaseqSession<'db> {
    eterner_db: &'db EternerDb,
    floater_db: FloaterDb,
}

impl<'db> VdBaseqSession<'db> {
    pub fn new(eterner_db: &'db EternerDb) -> Self {
        Self {
            eterner_db,
            floater_db: FloaterDb::default(),
        }
    }
}
