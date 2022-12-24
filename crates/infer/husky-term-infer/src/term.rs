mod error;

pub use error::*;

use crate::*;
use husky_expr::{AtomExpr, Expr, ExprIdx};
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
        let expr = self.expr();
        match expr {
            Expr::Atom(ref atom) => match atom {
                AtomExpr::Literal(_) => todo!(),
                AtomExpr::Symbol(symbol) => match symbol {
                    Symbol::Entity(_) => todo!(),
                    Symbol::Variable(_) => todo!(),
                    Symbol::Lifetime(_) => todo!(),
                    Symbol::Label(_) => todo!(),
                },
                AtomExpr::Unrecognized(ident) => Err(DerivedTermInferError::InferTermUnrecogized {
                    ident: self.db.dt_ident(*ident).to_owned(),
                }
                .into()),
                AtomExpr::Uncertain(_) => todo!(),
            },
            Expr::Opn {
                ref opn_variant,
                ref opds,
            } => todo!(),
        }
    }
}
