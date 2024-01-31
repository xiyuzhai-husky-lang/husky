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
use husky_term_prelude::literal::StringLiteralTokenData;

/// follows mainly from <https://doc.rust-lang.org/reference/tokens.html#literals/>
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
pub enum LiteralTokenData {
    Unit,
    Char(CharLiteralTokenData),
    String(StringLiteralTokenData),
    Integer(IntegerLikeLiteralTokenData),
    Float(FloatLiteralTokenData),
    TupleIndex(TupleIndexLiteralTokenData),
    Bool(BoolLiteralTokenData),
}

impl LiteralTokenData {
    pub fn negative(self, db: &::salsa::Db) -> Option<LiteralTokenData> {
        match self {
            LiteralTokenData::Unit => None,
            LiteralTokenData::Integer(i) => Some(LiteralTokenData::Integer(i.negative()?)),
            LiteralTokenData::Float(f) => Some(LiteralTokenData::Float(f.negative(db))),
            LiteralTokenData::Bool(_) => None,
            LiteralTokenData::String(_) => None,
            LiteralTokenData::Char(_) => None,
            LiteralTokenData::TupleIndex(_) => None,
        }
    }
}
