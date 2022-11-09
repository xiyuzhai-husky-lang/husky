use crate::*;
use husky_entity_path::EntityPathItd;
use husky_expr_syntax::{RawExpr, RawExprArena, RawExprIdx, RawExprVariant};
use husky_term::{TermAtom, TermContext, TermItd, TermMenu, TermOwned, Ty};
use husky_word::InternWord;

pub(crate) struct InferContext<'a> {
    db: &'a dyn TermInferDb,
    sheet: &'a mut TermSheet,
    expr_arena: &'a RawExprArena,
    expr: RawExprIdx,
    term_menu: &'a TermMenu,
}

impl<'a> InternWord for InferContext<'a> {
    fn word_itr(&self) -> &husky_word::WordInterner {
        self.db.word_itr()
    }
}

impl<'a> InferContext<'a> {
    pub(crate) fn new(
        db: &'a dyn TermInferDb,
        sheet: &'a mut TermSheet,
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

    pub(crate) fn subexpr_context<'b>(&'b mut self, subexpr: RawExprIdx) -> InferContext<'b>
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

    pub(crate) fn cached_term_result(&self) -> Option<&TermInferResult<TermItd>> {
        self.sheet.cached_term(self.expr)
    }
    pub(crate) fn cache_term_result(&mut self, term_result: TermInferResult<TermItd>) {
        self.sheet.cache_term(self.expr, term_result)
    }

    #[inline(always)]
    fn term_ctx(&self) -> TermContext<'a> {
        TermContext::new(self.db.upcast(), self.term_menu)
    }

    pub(crate) fn entity_path_term(&self, path: EntityPathItd) -> TermInferResult<TermItd> {
        self.term_ctx().entity_path_term(path).map_err(|e| e.into())
    }
}
