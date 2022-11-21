use husky_symbol_syntax::Symbol;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AtomExpr {
    Literal(RawLiteralData),
    Symbol(Symbol),
    Uncertain,
}

impl From<RawLiteralData> for AtomExpr {
    fn from(value: RawLiteralData) -> Self {
        AtomExpr::Literal(value)
    }
}

impl From<Symbol> for AtomExpr {
    fn from(symbol: Symbol) -> Self {
        AtomExpr::Symbol(symbol)
    }
}
