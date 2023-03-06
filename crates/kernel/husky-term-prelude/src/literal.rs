use crate::*;
use husky_token::StringLiteral;
use ordered_float::OrderedFloat;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = TermPreludeDb, jar = TermPreludeJar)]
pub enum TermLiteral {
    Unit,
    I32(i32),
    I64(i64),
    Nat(TermNaturalNumber),
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

impl TermLiteral {
    #[inline(always)]
    pub fn from_valid(db: &dyn TermPreludeDb, valid_term: TermLiteral) -> Self {
        todo!()
    }

    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn TermPreludeDb,
    ) -> std::fmt::Result {
        match self {
            TermLiteral::Unit => f.write_str("unit"),
            TermLiteral::I32(_) => todo!(),
            TermLiteral::I64(_) => todo!(),
            TermLiteral::Nat(_) => todo!(),
            TermLiteral::Float(_) => todo!(),
            TermLiteral::F32(_) => todo!(),
            TermLiteral::F64(_) => todo!(),
            TermLiteral::B8(_) => todo!(),
            TermLiteral::B16(_) => todo!(),
            TermLiteral::B32(_) => todo!(),
            TermLiteral::B64(_) => todo!(),
            TermLiteral::Bool(_) => todo!(),
            TermLiteral::Str(_) => todo!(),
            TermLiteral::EvalLifetime => f.write_str("'eval"),
            TermLiteral::StaticLifetime => f.write_str("'static"),
        }
    }
}

/// allowing representing very large number
#[salsa::interned(db = TermPreludeDb, jar = TermPreludeJar)]
pub struct TermInteger128 {
    pub value: i128,
}

/// allowing representing very large number
#[salsa::interned(db = TermPreludeDb, jar = TermPreludeJar)]
pub struct TermInteger256 {
    pub value: [i128; 2],
}

/// allowing representing very large number
#[salsa::interned(db = TermPreludeDb, jar = TermPreludeJar)]
pub struct TermNaturalNumber {
    pub bits: Vec<usize>,
}

impl<Db: TermPreludeDb + ?Sized> salsa::DisplayWithDb<Db> for TermLiteral {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<TermPreludeJar>>::as_jar_db(db);
        self.show_with_db_fmt(f, db)
    }
}
