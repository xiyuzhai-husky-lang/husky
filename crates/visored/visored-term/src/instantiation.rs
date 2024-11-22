pub mod menu;

use crate::term::VdTerm;
use lisp_csv::expr::LpCsvExpr;
use smallvec::SmallVec;
use visored_item_path::path::VdItemPath;

#[salsa::interned]
pub struct VdInstantiation {
    pub path: VdItemPath,
    #[return_ref]
    pub arguments: SmallVec<[VdTerm; 4]>,
}

impl VdInstantiation {
    pub fn from_lp_csv_expr(expr: &LpCsvExpr, db: &::salsa::Db) -> Self {
        let (path, args) = expr.application_expansion();
        let path = VdItemPath::from_lp_csv_expr(path);
        let arguments = args
            .iter()
            .map(|arg| VdTerm::from_lp_csv_expr(arg, db))
            .collect();
        Self::new(db, path, arguments)
    }
}
