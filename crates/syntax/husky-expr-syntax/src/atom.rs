use husky_symbol_syntax::Symbol;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RawExprAtomVariant {
    Literal(RawLiteralData),
    Symbol(Symbol),
}

impl From<Symbol> for RawExprAtomVariant {
    fn from(symbol: Symbol) -> Self {
        RawExprAtomVariant::Symbol(symbol)
    }
}
