use husky_symbol_syntax::Symbol;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RawAtom {
    Literal(RawLiteralData),
    Symbol(Symbol),
    Uncertain,
}

impl From<RawLiteralData> for RawAtom {
    fn from(value: RawLiteralData) -> Self {
        RawAtom::Literal(value)
    }
}

impl From<Symbol> for RawAtom {
    fn from(symbol: Symbol) -> Self {
        RawAtom::Symbol(symbol)
    }
}
