use crate::*;
use husky_expr_syntax::{RawExprArena, RawExprIdx, RawExprMap};
use husky_term::{TermPtr, Ty};

#[derive(Debug)]
pub struct TyInferSheet {
    ty_results: RawExprMap<InferResult<Ty>>,
    term_results: RawExprMap<InferResult<TermPtr>>,
}

impl TyInferSheet {
    pub(crate) fn new(arena: &RawExprArena) -> Self {
        Self {
            ty_results: RawExprMap::new(arena),
            term_results: RawExprMap::new(arena),
        }
    }

    pub(crate) fn insert_ty_infer_result(&mut self, expr: RawExprIdx, ty: InferResult<Ty>) {
        self.ty_results.insert_new(expr, ty)
    }

    pub(crate) fn insert_term_infer_result(
        &mut self,
        expr: RawExprIdx,
        term: InferResult<TermPtr>,
    ) {
        todo!()
    }
    pub(crate) fn cached_term(&self, expr: RawExprIdx) -> Option<&InferResult<TermPtr>> {
        self.term_results.get(expr)
    }

    pub(crate) fn cache_term(&mut self, expr: RawExprIdx, term_result: InferResult<TermPtr>) {
        self.term_results.insert_new(expr, term_result)
    }
}
