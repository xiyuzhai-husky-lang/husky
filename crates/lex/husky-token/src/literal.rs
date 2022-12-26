mod char_literal;
mod float_literal;
mod integer_literal;
mod string_literal;
mod tuple_index_literal;

pub use char_literal::*;
pub use float_literal::*;
pub use integer_literal::*;
pub use tuple_index_literal::*;

use crate::*;
use ordered_float::OrderedFloat;
use std::sync::Arc;

/// follows mainly from <https://doc.rust-lang.org/reference/tokens.html#literals/>

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum LiteralToken {
    Unit,
    Char(CharLiteral),
    String(StringLiteral),
    Integer(IntegerLiteral),
    Float(FloatLiteral),
    TupleIndex(TupleIndexLiteral),
    Bool(bool),
}
#[salsa::tracked(jar = TokenJar)]
pub struct StringLiteral {
    data: String,
}

impl LiteralToken {
    pub fn negative(self) -> Option<LiteralToken> {
        match self {
            LiteralToken::Unit => None,
            LiteralToken::Integer(i) => Some(LiteralToken::Integer(-i)),
            LiteralToken::Float(f) => Some(LiteralToken::Float(-f)),
            LiteralToken::Bool(_) => None,
            LiteralToken::String(_) => None,
            LiteralToken::Char(_) => todo!(),
            LiteralToken::TupleIndex(_) => todo!(),
        }
    }
}
