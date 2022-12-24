use crate::*;
use husky_entity_path::EntityPath;
use husky_expr::{Expr, ExprArena, ExprIdx};
use husky_term::{Term, TermAtom, TermContext, TermData, TermMenu};
use husky_word::WordDb;

pub(crate) struct InferContext<'a> {
    pub(crate) db: &'a dyn TermInferDb,
    sheet: &'a mut TermSheet,
    expr_arena: &'a ExprArena,
    expr: ExprIdx,
    term_menu: &'a TermMenu,
}

impl<'a> InferContext<'a> {
    pub(crate) fn new(
        db: &'a dyn TermInferDb,
        sheet: &'a mut TermSheet,
        expr_arena: &'a ExprArena,
        expr: ExprIdx,
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

    pub(crate) fn subexpr_context<'b>(&'b mut self, subexpr: ExprIdx) -> InferContext<'b>
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

    pub(crate) fn expr(&self) -> &'a Expr {
        &self.expr_arena[self.expr]
    }

    pub(crate) fn term_menu(&self) -> &'a TermMenu {
        self.term_menu
    }

    fn infer_entity_ty(&self, entity: husky_entity_path::EntityPath) -> husky_term::Term {
        self.db.entity_ty(entity)
    }

    pub(crate) fn cached_term_result(&self) -> Option<&TermInferResult<Term>> {
        self.sheet.cached_term(self.expr)
    }
    pub(crate) fn cache_term_result(&mut self, term_result: TermInferResult<Term>) {
        self.sheet.cache_term(self.expr, term_result)
    }

    #[inline(always)]
    fn term_ctx(&self) -> TermContext<'a> {
        TermContext::new(self.db.upcast(), self.term_menu)
    }

    pub(crate) fn entity_path_term(&self, path: EntityPath) -> TermInferResult<Term> {
        self.term_ctx().entity_path_term(path).map_err(|e| e.into())
    }
}
