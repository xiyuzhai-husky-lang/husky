mod error;

pub use error::*;

use crate::*;
use husky_expr::{Expr, ExprIdx};
use husky_symbol::Symbol;
use husky_term::Term;
use wild_utils::arb_ref;

impl<'a> InferContext<'a> {
    pub(crate) fn term_result<'b>(&'b mut self) -> Result<Term, &'b TermInferError> {
        match unsafe { arb_ref(self) }.cached_term_result() {
            Some(term_result) => match term_result {
                Ok(t) => Ok(*t),
                Err(e) => Err(e),
            },
            None => {
                let term_result = self.infer_term();
                self.cache_term_result(term_result);
                match self.cached_term_result().unwrap() {
                    Ok(t) => Ok(*t),
                    Err(e) => Err(e),
                }
            }
        }
    }

    fn infer_term(&mut self) -> TermInferResult<Term> {
        todo!()
        //     let expr = self.expr();
        //     match expr {
        //         Expr::Literal(_) => todo!(),
        //         Expr::EntityPath(_) => todo!(),
        //         Expr::Variable {
        //             token_idx,
        //             variable_idx,
        //         } => todo!(),
        //         Expr::Unrecognized(ident) => Err(DerivedTermInferError::InferTermUnrecogized {
        //             ident: self.db.dt_ident(*ident).to_owned(),
        //         }
        //         .into()),
        //         Expr::Uncertain(_) => todo!(),
        //         Expr::Opn {
        //             opn: ref opn_variant,
        //             ref opds,
        //         } => todo!(),
        //         Expr::Bracketed(_) => todo!(),
        //         Expr::Err(_) => todo!(),
        //         Expr::MethodCall {
        //             this_expr,
        //             arguments,
        //             lpar_token_idx,
        //             rpar_token_idx,
        //         } => todo!(),
        //     }
    }
}
