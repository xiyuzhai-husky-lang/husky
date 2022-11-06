mod error;

pub use error::*;

use crate::*;
use husky_expr_syntax::{RawAtom, RawExprIdx, RawExprVariant};
use husky_symbol_syntax::SymbolKind;
use husky_term::TermItd;
use wild_utils::arb_ref;

impl<'a> InferContext<'a> {
    pub(crate) fn term_result<'b>(&'b mut self) -> Result<TermItd, &'b TermInferError> {
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

    fn infer_term(&mut self) -> TermInferResult<TermItd> {
        let expr = self.expr();
        match expr.variant {
            RawExprVariant::Atom(ref atom) => match atom {
                RawAtom::Literal(_) => todo!(),
                RawAtom::Symbol(symbol) => match symbol.kind {
                    SymbolKind::EntityPath(path) => self.entity_path_term(path),
                    SymbolKind::LocalVariable { init_range } => todo!(),
                    SymbolKind::FrameVariable { init_range } => todo!(),
                    SymbolKind::Unrecognized => Err(
                        DerivedTermInferError::InferTermUnrecogizedSymbol { symbol: *symbol }
                            .into(),
                    ),
                    SymbolKind::ThisValue => todo!(),
                    SymbolKind::ThisMethod => todo!(),
                    SymbolKind::ThisField => todo!(),
                },
                RawAtom::Uncertain => todo!(),
            },
            RawExprVariant::Bracketed(_) => todo!(),
            RawExprVariant::Opn {
                ref opn_variant,
                ref opds,
            } => todo!(),
            RawExprVariant::Lambda(_, _) => todo!(),
        }
    }
}
