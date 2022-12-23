use crate::*;
use husky_symbol_syntax::Symbol;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AtomExpr {
    Literal(LiteralToken),
    Symbol(Symbol),
    Uncertain(Identifier),
    Unrecognized(Identifier),
}

impl From<LiteralToken> for AtomExpr {
    fn from(value: LiteralToken) -> Self {
        AtomExpr::Literal(value)
    }
}

impl From<Symbol> for AtomExpr {
    fn from(symbol: Symbol) -> Self {
        AtomExpr::Symbol(symbol)
    }
}
