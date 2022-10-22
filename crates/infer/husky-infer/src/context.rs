use crate::*;
use husky_expr_syntax::{RawExpr, RawExprArena, RawExprIdx, RawExprVariant};
use husky_term::{TermItd, TermMenu, Ty};
use husky_word::InternWord;

pub(crate) struct TyInferContext<'a> {
    db: &'a dyn TyInferDb,
    sheet: &'a mut TyInferSheet,
    expr_arena: &'a RawExprArena,
    expr: RawExprIdx,
    term_menu: &'a TermMenu,
}

impl<'a> InternWord for TyInferContext<'a> {
    fn word_itr(&self) -> &husky_word::WordInterner {
        self.db.word_itr()
    }
}

impl<'a> TyInferContext<'a> {
    pub(crate) fn new(
        db: &'a dyn TyInferDb,
        sheet: &'a mut TyInferSheet,
        expr_arena: &'a RawExprArena,
        expr: RawExprIdx,
        term_menu: &'a TermMenu,
    ) -> Self {
        Self {
            db,
            sheet,
            expr_arena,
            expr,
            term_menu,
        }
    }

    pub(crate) fn subexpr_context<'b>(&'b mut self, subexpr: RawExprIdx) -> TyInferContext<'b>
    where
        'a: 'b,
    {
        Self {
            db: self.db,
            sheet: unsafe { &mut *(self.sheet as *mut _) },
            expr_arena: self.expr_arena,
            expr: subexpr,
            term_menu: self.term_menu,
        }
    }

    pub(crate) fn run(mut self) {
        let ty = self.infer();
        self.sheet.insert_ty_infer_result(self.expr, ty)
    }

    pub(crate) fn expr(&self) -> &'a RawExpr {
        &self.expr_arena[self.expr]
    }

    pub(crate) fn term_menu(&self) -> &'a TermMenu {
        self.term_menu
    }

    fn infer_entity_ty(&self, entity: husky_entity_path::EntityPathItd) -> husky_term::Ty {
        self.db.entity_ty(entity)
    }

    pub(crate) fn cached_term_result(&self) -> Option<&InferResult<TermItd>> {
        self.sheet.cached_term(self.expr)
    }
    pub(crate) fn cache_term_result(&mut self, term_result: InferResult<TermItd>) {
        self.sheet.cache_term(self.expr, term_result)
    }
}
