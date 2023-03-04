use crate::*;
use husky_token::StringLiteral;
use ordered_float::OrderedFloat;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = PreciseTermDb, jar = PreciseTermJar)]
pub enum PreciseTermLiteral {
    Unit,
    I32(i32),
    I64(i64),
    Nat(PreciseTermNaturalNumber),
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

impl PreciseTermLiteral {
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn PreciseTermDb,
        ctx: &mut PreciseTermShowContext,
    ) -> std::fmt::Result {
        match self {
            PreciseTermLiteral::Unit => f.write_str("unit"),
            PreciseTermLiteral::I32(_) => todo!(),
            PreciseTermLiteral::I64(_) => todo!(),
            PreciseTermLiteral::Nat(_) => todo!(),
            PreciseTermLiteral::Float(_) => todo!(),
            PreciseTermLiteral::F32(_) => todo!(),
            PreciseTermLiteral::F64(_) => todo!(),
            PreciseTermLiteral::B8(_) => todo!(),
            PreciseTermLiteral::B16(_) => todo!(),
            PreciseTermLiteral::B32(_) => todo!(),
            PreciseTermLiteral::B64(_) => todo!(),
            PreciseTermLiteral::Bool(_) => todo!(),
            PreciseTermLiteral::Str(_) => todo!(),
            PreciseTermLiteral::EvalLifetime => f.write_str("'eval"),
            PreciseTermLiteral::StaticLifetime => f.write_str("'static"),
        }
    }
}

impl<Db: PreciseTermDb + ?Sized> salsa::DisplayWithDb<Db> for PreciseTermLiteral {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<PreciseTermJar>>::as_jar_db(db);
        self.show_with_db_fmt(f, db, &mut Default::default())
    }
}

/// allowing representing very large number
#[salsa::interned(db = PreciseTermDb, jar = PreciseTermJar)]
pub struct PreciseTermInteger128 {
    pub value: i128,
}

/// allowing representing very large number
#[salsa::interned(db = PreciseTermDb, jar = PreciseTermJar)]
pub struct PreciseTermInteger256 {
    pub value: [i128; 2],
}

/// allowing representing very large number
#[salsa::interned(db = PreciseTermDb, jar = PreciseTermJar)]
pub struct PreciseTermNaturalNumber {
    pub bits: Vec<usize>,
}

impl From<i32> for PreciseTerm {
    fn from(value: i32) -> Self {
        PreciseTerm::Literal(value.into())
    }
}

impl From<i64> for PreciseTerm {
    fn from(value: i64) -> Self {
        PreciseTerm::Literal(value.into())
    }
}

impl std::fmt::Display for PreciseTermLiteral {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl From<i32> for PreciseTermLiteral {
    fn from(value: i32) -> Self {
        PreciseTermLiteral::I32(value)
    }
}

impl From<i64> for PreciseTermLiteral {
    fn from(value: i64) -> Self {
        PreciseTermLiteral::I64(value)
    }
}
