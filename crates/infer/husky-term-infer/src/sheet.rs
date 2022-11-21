use crate::*;
use husky_ast::AstText;
use husky_expr_syntax::{ExprArena, ExprIdx, ExprMap};
use husky_term::{TermItd, Ty};

#[derive(Debug, PartialEq, Eq)]
pub struct TermSheet {
    ty_results: ExprMap<TermInferResult<Ty>>,
    term_results: ExprMap<TermInferResult<TermItd>>,
}

impl TermSheet {
    pub(crate) fn new(arena: &ExprArena) -> Self {
        Self {
            ty_results: ExprMap::new(arena),
            term_results: ExprMap::new(arena),
        }
    }

    pub(crate) fn insert_ty_infer_result(&mut self, expr: ExprIdx, ty: TermInferResult<Ty>) {
        self.ty_results.insert_new(expr, ty)
    }

    pub(crate) fn insert_term_infer_result(
        &mut self,
        expr: ExprIdx,
        term: TermInferResult<TermItd>,
    ) {
        todo!()
    }
    pub(crate) fn cached_term(&self, expr: ExprIdx) -> Option<&TermInferResult<TermItd>> {
        self.term_results.get(expr)
    }

    pub(crate) fn cache_term(&mut self, expr: ExprIdx, term_result: TermInferResult<TermItd>) {
        self.term_results.insert_new(expr, term_result)
    }
}

impl TermSheet {
    pub fn ast_text(&self) -> &AstText {
        todo!()
    }

    pub fn expr_ty_result(&self, expr: ExprIdx) -> &TermInferResult<Ty> {
        todo!()
    }
}
