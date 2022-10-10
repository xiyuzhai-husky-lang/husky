use husky_symbol_syntax::Symbol;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RawAtom {
    Literal(RawLiteralData),
    Symbol(Symbol),
}

impl From<Symbol> for RawAtom {
    fn from(symbol: Symbol) -> Self {
        RawAtom::Symbol(symbol)
    }
}
