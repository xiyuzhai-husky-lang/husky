use crate::*;
use husky_expr_syntax::{RawExpr, RawExprArena, RawExprIdx, RawExprVariant};
use husky_term::{TermMenu, Ty};

pub(crate) struct TyInferContext<'a> {
    db: &'a dyn TyInferDb,
    sheet: &'a mut TyInferSheet,
    arena: &'a RawExprArena,
    expr: RawExprIdx,
    term_menu: &'a TermMenu,
}

impl<'a> TyInferContext<'a> {
    pub(crate) fn new(
        db: &'a dyn TyInferDb,
        sheet: &'a mut TyInferSheet,
        arena: &'a RawExprArena,
        expr: RawExprIdx,
        term_menu: &'a TermMenu,
    ) -> Self {
        Self {
            db,
            sheet,
            arena,
            expr,
            term_menu,
        }
    }

    pub(crate) fn run(mut self) {
        let ty = self.infer();
        self.sheet.insert(self.expr, ty)
    }

    pub(crate) fn expr(&self) -> &'a RawExpr {
        &self.arena[self.expr]
    }

    pub(crate) fn term_menu(&self) -> &'a TermMenu {
        self.term_menu
    }

    fn infer_entity_ty(&self, entity: husky_entity_path::EntityPathPtr) -> husky_term::Ty {
        self.db.entity_ty(entity)
    }
}
