mod bool;
mod char;
mod float;
mod integer;
mod string;
mod tuple_index;

pub use self::bool::*;
pub use self::char::*;
pub use self::float::*;
pub use self::integer::*;
pub use self::string::*;
pub use self::tuple_index::*;

use super::*;
use crate::TokenJar;

/// follows mainly from <https://doc.rust-lang.org/reference/tokens.html#literals/>
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = TokenDb)]
pub enum Literal {
    Unit,
    Char(CharLiteral),
    String(StringLiteral),
    Integer(IntegerLikeLiteral),
    Float(FloatLiteral),
    TupleIndex(TupleIndexLiteral),
    Bool(BoolLiteral),
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
