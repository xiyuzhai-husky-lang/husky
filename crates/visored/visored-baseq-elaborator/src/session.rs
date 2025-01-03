use crate::tactic::VdBaseqTactic;
use crate::*;
use floated_sequential::db::FloaterDb;
use strategy::obvious::load_obvious_tactics;
use visored_mir_expr::expr::VdMirExprIdx;

type SuperVarsContext = ();
type Term = ();

pub struct VdBaseqSession<'db> {
    eterner_db: &'db EternerDb,
    floater_db: FloaterDb,
    obvious_tactics: Vec<VdBaseqTactic>,
}

impl<'db> VdBaseqSession<'db> {
    pub fn new(eterner_db: &'db EternerDb) -> Self {
        Self {
            eterner_db,
            floater_db: FloaterDb::default(),
            obvious_tactics: load_obvious_tactics(),
        }
    }
}

impl<'db> VdBaseqSession<'db> {
    pub fn eterner_db(&self) -> &'db EternerDb {
        self.eterner_db
    }

    pub fn floater_db(&self) -> &FloaterDb {
        &self.floater_db
    }

    pub fn obvious_tactics(&self) -> &[VdBaseqTactic] {
        &self.obvious_tactics
    }
}
