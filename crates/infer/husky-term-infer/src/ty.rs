use crate::*;
use husky_expr::{ExprIdx, ExprIdxRange};
use husky_print_utils::p;
use husky_term::Term;

impl<'a> InferContext<'a> {
    pub(crate) fn infer(&mut self) -> TermInferResult<Term> {
        todo!()
    }

    fn infer_subexpr(&mut self, subexpr: ExprIdx) -> TermInferResult<Term> {
        self.subexpr_context(subexpr).infer()
    }

    fn infer_literal(&self, literal: &Literal) -> Term {
        let term_menu = self.term_menu();
        match literal {
            Literal::Unit => todo!(),
            Literal::Integer(_) => term_menu.i32(),
            Literal::Float(_) => todo!(),
            Literal::Bool(_) => todo!(),
            Literal::String(_) => todo!(),
            Literal::Char(_) => todo!(),
            Literal::TupleIndex(_) => todo!(),
        }
    }
}
