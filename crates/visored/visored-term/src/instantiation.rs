pub mod menu;

use crate::term::VdTerm;
use eterned::db::{attached_interner_db, EternerDb};
use lisp_csv::expr::LpCsvExpr;
use smallvec::SmallVec;
use visored_entity_path::path::VdItemPath;

#[eterned::eterned]
pub struct VdInstantiation {
    pub path: VdItemPath,
    pub arguments: SmallVec<[VdTerm; 4]>,
}

impl std::fmt::Debug for VdInstantiation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let db = attached_interner_db().ok_or(std::fmt::Error)?;
        write!(f, "{:?} ...", self.path(db))
    }
}

impl VdInstantiation {
    pub fn from_lp_csv_expr(expr: &LpCsvExpr, db: &EternerDb) -> Self {
        let (path, args) = expr.application_expansion();
        let path = VdItemPath::from_lp_csv_expr(path, db);
        let arguments = args
            .iter()
            .map(|arg| VdTerm::from_lp_csv_expr(arg, db))
            .collect();
        Self::new(path, arguments, db)
    }
}
