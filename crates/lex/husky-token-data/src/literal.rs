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

pub use self::tuple_index::*;

use crate::*;

/// follows mainly from <https://doc.rust-lang.org/reference/tokens.html#literals/>
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = TokenDataDb)]
pub enum LiteralData {
    Unit,
    Char(CharLiteralData),
    String(StringLiteralData),
    Integer(IntegerLikeLiteralData),
    Float(FloatLiteralData),
    TupleIndex(TupleIndexLiteralData),
    Bool(BoolLiteralData),
}

impl LiteralData {
    pub fn negative(self) -> Option<LiteralData> {
        match self {
            LiteralData::Unit => None,
            LiteralData::Integer(i) => Some(LiteralData::Integer(-i)),
            LiteralData::Float(f) => Some(LiteralData::Float(-f)),
            LiteralData::Bool(_) => None,
            LiteralData::String(_) => None,
            LiteralData::Char(_) => todo!(),
            LiteralData::TupleIndex(_) => todo!(),
        }
    }
}
