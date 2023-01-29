mod bool_literal;
mod char_literal;
mod float_literal;
mod integer_literal;
mod string_literal;
mod tuple_index_literal;

pub use bool_literal::*;
pub use char_literal::*;
pub use float_literal::*;
pub use integer_literal::*;
pub use tuple_index_literal::*;

use super::*;
use crate::TokenJar;

/// follows mainly from <https://doc.rust-lang.org/reference/tokens.html#literals/>
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Literal {
    Unit,
    Char(CharLiteral),
    String(StringLiteral),
    Integer(IntegerLiteral),
    Float(FloatLiteral),
    TupleIndex(TupleIndexLiteral),
    Bool(BoolLiteral),
}

#[salsa::tracked(db = TokenDb, jar = TokenJar)]
pub struct StringLiteral {
    data: String,
}

impl Literal {
    pub fn negative(self) -> Option<Literal> {
        match self {
            Literal::Unit => None,
            Literal::Integer(i) => Some(Literal::Integer(-i)),
            Literal::Float(f) => Some(Literal::Float(-f)),
            Literal::Bool(_) => None,
            Literal::String(_) => None,
            Literal::Char(_) => todo!(),
            Literal::TupleIndex(_) => todo!(),
        }
    }
}
