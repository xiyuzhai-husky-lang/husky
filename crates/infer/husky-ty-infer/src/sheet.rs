use crate::*;
use husky_expr_syntax::{RawExprArena, RawExprIdx, RawExprMap};
use husky_term::Ty;

#[derive(Debug)]
pub struct TyInferSheet {
    infer_results: RawExprMap<TyInferResult<Ty>>,
}

impl TyInferSheet {
    pub(crate) fn new(arena: &RawExprArena) -> Self {
        Self {
            infer_results: RawExprMap::new(arena),
        }
    }

    pub(crate) fn insert(&mut self, expr: RawExprIdx, ty: TyInferResult<Ty>) {
        self.infer_results.insert_new(expr, ty)
    }
}
