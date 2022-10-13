use crate::*;
use husky_expr_syntax::{RawExprArena, RawExprIdx};

pub(crate) struct TyInferContext<'a> {
    db: &'a dyn TyInferDb,
    sheet: &'a mut TyInferSheet,
    arena: &'a RawExprArena,
    basepoint: RawExprIdx,
}

impl<'a> TyInferContext<'a> {
    pub(crate) fn new(
        db: &'a dyn TyInferDb,
        sheet: &'a mut TyInferSheet,
        arena: &'a RawExprArena,
        basepoint: RawExprIdx,
    ) -> Self {
        Self {
            db,
            sheet,
            arena,
            basepoint,
        }
    }

    fn infer_entity_ty(&self, entity: husky_entity_path::EntityPathPtr) -> husky_term::Ty {
        self.db.entity_ty(entity)
    }
}
