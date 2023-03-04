use crate::*;
use husky_token::StringLiteral;
use ordered_float::OrderedFloat;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = ValidTermDb, jar = ValidTermJar)]
pub enum ValidTermLiteral {
    Unit,
    I32(i32),
    I64(i64),
    Nat(ValidTermNaturalNumber),
    Float(OrderedFloat<f64>),
    F32(OrderedFloat<f32>),
    F64(OrderedFloat<f64>),
    B8(u8),
    B16(u16),
    B32(u32),
    B64(u64),
    Bool(bool),
    Str(StringLiteral),
    EvalLifetime,
    StaticLifetime,
}

impl ValidTermLiteral {
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn ValidTermDb,
        ctx: &mut ValidTermShowContext,
    ) -> std::fmt::Result {
        match self {
            ValidTermLiteral::Unit => f.write_str("unit"),
            ValidTermLiteral::I32(_) => todo!(),
            ValidTermLiteral::I64(_) => todo!(),
            ValidTermLiteral::Nat(_) => todo!(),
            ValidTermLiteral::Float(_) => todo!(),
            ValidTermLiteral::F32(_) => todo!(),
            ValidTermLiteral::F64(_) => todo!(),
            ValidTermLiteral::B8(_) => todo!(),
            ValidTermLiteral::B16(_) => todo!(),
            ValidTermLiteral::B32(_) => todo!(),
            ValidTermLiteral::B64(_) => todo!(),
            ValidTermLiteral::Bool(_) => todo!(),
            ValidTermLiteral::Str(_) => todo!(),
            ValidTermLiteral::EvalLifetime => f.write_str("'eval"),
            ValidTermLiteral::StaticLifetime => f.write_str("'static"),
        }
    }
}

impl<Db: ValidTermDb + ?Sized> salsa::DisplayWithDb<Db> for ValidTermLiteral {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<ValidTermJar>>::as_jar_db(db);
        self.show_with_db_fmt(f, db, &mut Default::default())
    }
}

/// allowing representing very large number
#[salsa::interned(db = ValidTermDb, jar = ValidTermJar)]
pub struct ValidTermInteger128 {
    pub value: i128,
}

/// allowing representing very large number
#[salsa::interned(db = ValidTermDb, jar = ValidTermJar)]
pub struct ValidTermInteger256 {
    pub value: [i128; 2],
}

/// allowing representing very large number
#[salsa::interned(db = ValidTermDb, jar = ValidTermJar)]
pub struct ValidTermNaturalNumber {
    pub bits: Vec<usize>,
}

impl From<i32> for ValidTerm {
    fn from(value: i32) -> Self {
        ValidTerm::Literal(value.into())
    }
}

impl From<i64> for ValidTerm {
    fn from(value: i64) -> Self {
        ValidTerm::Literal(value.into())
    }
}

impl std::fmt::Display for ValidTermLiteral {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl From<i32> for ValidTermLiteral {
    fn from(value: i32) -> Self {
        ValidTermLiteral::I32(value)
    }
}

impl From<i64> for ValidTermLiteral {
    fn from(value: i64) -> Self {
        ValidTermLiteral::I64(value)
    }
}
