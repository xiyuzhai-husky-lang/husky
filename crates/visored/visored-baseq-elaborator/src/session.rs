use crate::tactic::VdBsqTactic;
use crate::*;
use floated_sequential::db::FloaterDb;
use strategy::obvious::load_obvious_tactics;
use visored_mir_expr::expr::VdMirExprIdx;

type SuperVarsContext = ();
type Term = ();

pub struct VdBsqSession<'db> {
    eterner_db: &'db EternerDb,
    floater_db: FloaterDb,
    obvious_tactics: Vec<VdBsqTactic>,
}

impl<'db> VdBsqSession<'db> {
    pub fn new(eterner_db: &'db EternerDb) -> Self {
        Self {
            eterner_db,
            floater_db: FloaterDb::default(),
            obvious_tactics: load_obvious_tactics(),
        }
    }
}

impl<'db> VdBsqSession<'db> {
    pub fn eterner_db(&self) -> &'db EternerDb {
        self.eterner_db
    }

    pub fn floater_db(&self) -> &FloaterDb {
        &self.floater_db
    }

    pub fn obvious_tactics(&self) -> &[VdBsqTactic] {
        &self.obvious_tactics
    }
}
