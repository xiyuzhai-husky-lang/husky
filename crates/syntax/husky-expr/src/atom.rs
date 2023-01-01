use crate::*;
use husky_symbol::Symbol;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AtomExpr {
    Literal(TokenIdx),
    Symbol(Symbol),
    Uncertain(Identifier),
    Unrecognized(Identifier),
}

// impl From<Literal> for AtomExpr {
//     fn from(value: Literal) -> Self {
//         AtomExpr::Literal(value)
//     }
// }

impl From<Symbol> for AtomExpr {
    fn from(symbol: Symbol) -> Self {
        AtomExpr::Symbol(symbol)
    }
}
