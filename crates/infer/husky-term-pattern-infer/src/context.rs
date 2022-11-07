use crate::*;
use husky_entity_path::EntityPathItd;
use husky_expr_syntax::{RawExpr, RawExprArena, RawExprIdx, RawExprVariant};
use husky_term::{Term, TermAtom, TermContext, TermItd, TermMenu, Ty};
use husky_word::InternWord;

pub(crate) struct TermPatternInferContext<'a> {
    db: &'a dyn TermPatternInferDb,
    sheet: &'a mut TermPatternInferSheet,
    expr_arena: &'a RawExprArena,
    expr: RawExprIdx,
    term_menu: &'a TermMenu,
}

impl<'a> InternWord for TermPatternInferContext<'a> {
    fn word_itr(&self) -> &husky_word::WordInterner {
        self.db.word_itr()
    }
}

impl<'a> TermPatternInferContext<'a> {
    pub(crate) fn new(
        db: &'a dyn TermPatternInferDb,
        sheet: &'a mut TermPatternInferSheet,
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

    pub(crate) fn subexpr_context<'b>(
        &'b mut self,
        subexpr: RawExprIdx,
    ) -> TermPatternInferContext<'b>
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
        todo!()
        // self.sheet.insert_ty_infer_result(self.expr, ty)
    }

    pub(crate) fn expr(&self) -> &'a RawExpr {
        &self.expr_arena[self.expr]
    }

    pub(crate) fn term_menu(&self) -> &'a TermMenu {
        self.term_menu
    }

    #[inline(always)]
    fn term_ctx(&self) -> TermContext<'a> {
        TermContext::new(self.db.upcast(), self.term_menu)
    }

    pub(crate) fn entity_path_term(&self, path: EntityPathItd) -> TermPatternInferResult<TermItd> {
        self.map_original(self.term_ctx().entity_path_term(path))
    }
}
