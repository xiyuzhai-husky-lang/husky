use crate::*;
use husky_token::StringLiteral;
use ordered_float::OrderedFloat;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = RawTermDb, jar = RawTermJar)]
pub enum RawTermLiteral {
    Unit,
    I32(i32),
    I64(i64),
    Nat(RawTermNaturalNumber),
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

impl RawTermLiteral {
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn RawTermDb,
        ctx: &mut RawTermShowContext,
    ) -> std::fmt::Result {
        match self {
            RawTermLiteral::Unit => f.write_str("unit"),
            RawTermLiteral::I32(_) => todo!(),
            RawTermLiteral::I64(_) => todo!(),
            RawTermLiteral::Nat(_) => todo!(),
            RawTermLiteral::Float(_) => todo!(),
            RawTermLiteral::F32(_) => todo!(),
            RawTermLiteral::F64(_) => todo!(),
            RawTermLiteral::B8(_) => todo!(),
            RawTermLiteral::B16(_) => todo!(),
            RawTermLiteral::B32(_) => todo!(),
            RawTermLiteral::B64(_) => todo!(),
            RawTermLiteral::Bool(_) => todo!(),
            RawTermLiteral::Str(_) => todo!(),
            RawTermLiteral::EvalLifetime => f.write_str("'eval"),
            RawTermLiteral::StaticLifetime => f.write_str("'static"),
        }
    }
}

impl<Db: RawTermDb + ?Sized> salsa::DisplayWithDb<Db> for RawTermLiteral {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<RawTermJar>>::as_jar_db(db);
        self.show_with_db_fmt(f, db, &mut Default::default())
    }
}

/// allowing representing very large number
#[salsa::interned(db = RawTermDb, jar = RawTermJar)]
pub struct RawTermInteger128 {
    pub value: i128,
}

/// allowing representing very large number
#[salsa::interned(db = RawTermDb, jar = RawTermJar)]
pub struct RawTermInteger256 {
    pub value: [i128; 2],
}

/// allowing representing very large number
#[salsa::interned(db = RawTermDb, jar = RawTermJar)]
pub struct RawTermNaturalNumber {
    pub bits: Vec<usize>,
}

impl From<i32> for RawTerm {
    fn from(value: i32) -> Self {
        RawTerm::Literal(value.into())
    }
}

impl From<i64> for RawTerm {
    fn from(value: i64) -> Self {
        RawTerm::Literal(value.into())
    }
}

impl std::fmt::Display for RawTermLiteral {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl From<i32> for RawTermLiteral {
    fn from(value: i32) -> Self {
        RawTermLiteral::I32(value)
    }
}

impl From<i64> for RawTermLiteral {
    fn from(value: i64) -> Self {
        RawTermLiteral::I64(value)
    }
}
