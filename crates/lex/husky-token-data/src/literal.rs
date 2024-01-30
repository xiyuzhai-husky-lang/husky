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
use husky_term_prelude::literal::StringLiteralData;

/// follows mainly from <https://doc.rust-lang.org/reference/tokens.html#literals/>
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
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
    pub fn negative(self, db: &::salsa::Db) -> Option<LiteralData> {
        match self {
            LiteralData::Unit => None,
            LiteralData::Integer(i) => Some(LiteralData::Integer(i.negative()?)),
            LiteralData::Float(f) => Some(LiteralData::Float(f.negative(db))),
            LiteralData::Bool(_) => None,
            LiteralData::String(_) => None,
            LiteralData::Char(_) => None,
            LiteralData::TupleIndex(_) => None,
        }
    }
}
