use husky_expr_syntax::RawExprArena;
use husky_term_infer::{TermInferDb, TermSheet};

use super::*;

pub(crate) struct EagerParser<'a> {
    pub(super) db: &'a dyn TermInferDb,
    pub(super) arena: &'a RawExprArena,
    pub(super) file: PathItd,
    pub(super) target_entrance: PathItd,
    term_sheet: Arc<TermSheet>,
}

impl<'a> EagerParser<'a> {
    pub(crate) fn new(_db: &'a dyn TermInferDb, _arena: &'a RawExprArena, _file: PathItd) -> Self {
        todo!()
        // msg_once!("check no errors in entity_route_sheet");
        // let qualified_ty_sheet = db.qualified_ty_sheet(file).unwrap();
        // let target_entrance = db.module_target_entrance(file).unwrap();
        // Self {
        //     target_entrance,
        //     db,
        //     arena,
        //     file,
        //     term_sheet: db.term_sheet(file).unwrap(),
        // }
    }
}
