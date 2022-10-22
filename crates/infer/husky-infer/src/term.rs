use crate::*;
use husky_expr_syntax::{RawAtom, RawExprIdx, RawExprVariant};
use husky_symbol_syntax::SymbolKind;
use husky_term::TermItd;

impl<'a> TyInferContext<'a> {
    pub(crate) fn term_result<'b>(&'b mut self) -> Result<TermItd, ()> {
        match self.cached_term_result() {
            Some(term_result) => match term_result {
                Ok(t) => Ok(*t),
                Err(_) => Err(()),
            },
            None => {
                let term_result = self.infer_term();
                let term = match term_result {
                    Ok(term) => Ok(term),
                    Err(_) => Err(()),
                };
                self.cache_term_result(term_result);
                term
            }
        }
    }

    fn infer_term(&mut self) -> InferResult<TermItd> {
        let expr = self.expr();
        match expr.variant {
            RawExprVariant::Atom(ref atom) => match atom {
                RawAtom::Literal(_) => todo!(),
                RawAtom::Symbol(symbol) => match symbol.kind {
                    SymbolKind::EntityPath(_) => todo!(),
                    SymbolKind::LocalVariable { init_range } => todo!(),
                    SymbolKind::FrameVariable { init_range } => todo!(),
                    SymbolKind::Unrecognized => Err(InferError::Derived),
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
