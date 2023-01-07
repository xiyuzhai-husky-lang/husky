use crate::*;
use husky_ast::AstSheet;
use husky_expr::{ExprIdx, ExprMap};
use husky_term::Term;

#[derive(Debug, PartialEq, Eq)]
pub struct TermSheet {
    ty_results: ExprMap<TermInferResult<Term>>,
    term_results: ExprMap<TermInferResult<Term>>,
}

impl TermSheet {
    pub(crate) fn new() -> Self {
        todo!()
        // Self {
        //     ty_results: ExprMap::new(sheet.expr_arena()),
        //     term_results: ExprMap::new(sheet.expr_arena()),
        // }
    }

    pub(crate) fn insert_ty_infer_result(&mut self, expr: ExprIdx, ty: TermInferResult<Term>) {
        self.ty_results.insert_new(expr, ty)
    }

    pub(crate) fn insert_term_infer_result(&mut self, expr: ExprIdx, term: TermInferResult<Term>) {
        todo!()
    }
    pub(crate) fn cached_term(&self, expr: ExprIdx) -> Option<&TermInferResult<Term>> {
        self.term_results.get(expr)
    }

    pub(crate) fn cache_term(&mut self, expr: ExprIdx, term_result: TermInferResult<Term>) {
        self.term_results.insert_new(expr, term_result)
    }
}

impl TermSheet {
    pub fn ast_text(&self) -> &AstSheet {
        todo!()
    }

    pub fn expr_ty_result(&self, expr: ExprIdx) -> &TermInferResult<Term> {
        todo!()
    }
}
