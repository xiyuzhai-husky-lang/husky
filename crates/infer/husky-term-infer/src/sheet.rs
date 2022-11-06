use crate::*;
use husky_ast::AstText;
use husky_expr_syntax::{RawExprArena, RawExprIdx, RawExprMap};
use husky_term::{TermItd, Ty};

#[derive(Debug, PartialEq, Eq)]
pub struct TermSheet {
    ty_results: RawExprMap<TermInferResult<Ty>>,
    term_results: RawExprMap<TermInferResult<TermItd>>,
}

impl TermSheet {
    pub(crate) fn new(arena: &RawExprArena) -> Self {
        Self {
            ty_results: RawExprMap::new(arena),
            term_results: RawExprMap::new(arena),
        }
    }

    pub(crate) fn insert_ty_infer_result(&mut self, expr: RawExprIdx, ty: TermInferResult<Ty>) {
        self.ty_results.insert_new(expr, ty)
    }

    pub(crate) fn insert_term_infer_result(
        &mut self,
        expr: RawExprIdx,
        term: TermInferResult<TermItd>,
    ) {
        todo!()
    }
    pub(crate) fn cached_term(&self, expr: RawExprIdx) -> Option<&TermInferResult<TermItd>> {
        self.term_results.get(expr)
    }

    pub(crate) fn cache_term(&mut self, expr: RawExprIdx, term_result: TermInferResult<TermItd>) {
        self.term_results.insert_new(expr, term_result)
    }
}

impl TermSheet {
    pub fn ast_text(&self) -> &AstText {
        todo!()
    }

    pub fn expr_ty_result(&self, expr: RawExprIdx) -> &TermInferResult<Ty> {
        todo!()
    }
}
