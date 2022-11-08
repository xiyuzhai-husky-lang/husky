use husky_symbol_syntax::Symbol;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RawAtomExpr {
    Literal(RawLiteralData),
    Symbol(Symbol),
    Uncertain,
}

impl From<RawLiteralData> for RawAtomExpr {
    fn from(value: RawLiteralData) -> Self {
        RawAtomExpr::Literal(value)
    }
}

impl From<Symbol> for RawAtomExpr {
    fn from(symbol: Symbol) -> Self {
        RawAtomExpr::Symbol(symbol)
    }
}
