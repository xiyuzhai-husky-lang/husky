use crate::*;
use husky_token::StringLiteral;
use ordered_float::OrderedFloat;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = TermPreludeDb, jar = TermPreludeJar)]
pub enum TermLiteral {
    /// unit literal
    Unit,
    /// boolean literal
    Bool(bool),
    /// 8-bit integer literal
    I8(i8),
    /// 16-bit integer literal
    I16(i16),
    /// 32-bit integer literal
    I32(i32),
    /// 64-bit integer literal
    I64(i64),
    /// 128-bit integer literal
    I128(i128),
    /// for cross compilation
    ISize(i64),
    /// 8-bit unsigned integer literal
    U8(u8),
    /// 16-bit unsigned integer liteUal
    U16(u16),
    /// 32-bit unsigned integer liteUal
    U32(u32),
    /// 64-bit unsigned integer liteUal
    U64(u64),
    /// 128-bit unsigned integer liteUal
    U128(u128),
    /// for cross compilation
    USize(u64),
    /// 8-bit raw bit literal
    R8(u8),
    /// 16-bit raw bit literal
    R16(u16),
    /// 32-bit raw bit literal
    R32(u32),
    /// 64-bit raw bit literal
    R64(u64),
    /// 128-bit raw bit literal
    R128(u128),
    /// for cross compilation
    RSize(u64),
    /// natural number literal
    Nat(TermNaturalNumber),
    /// 32-bit float literal
    F32(OrderedFloat<f32>),
    /// 64-bit float literal
    F64(OrderedFloat<f64>),
    /// string literal
    String(StringLiteral),
    /// eval lifetime
    EvalLifetime,
    /// static lifetime
    StaticLifetime,
}

impl TermLiteral {
    #[inline(always)]
    pub fn from_valid(db: &dyn TermPreludeDb, valid_term: TermLiteral) -> Self {
        todo!()
    }

    pub fn ty(self) -> PreludeTypePath {
        match self {
            TermLiteral::Unit => PreludeTypePath::Unit,
            TermLiteral::Bool(_) => PreludeTypePath::Bool,
            TermLiteral::I8(_) => PreludeTypePath::I8,
            TermLiteral::I16(_) => PreludeTypePath::I16,
            TermLiteral::I32(_) => PreludeTypePath::I32,
            TermLiteral::I64(_) => PreludeTypePath::I64,
            TermLiteral::I128(_) => PreludeTypePath::I128,
            TermLiteral::ISize(_) => PreludeTypePath::ISize,
            TermLiteral::U8(_) => PreludeTypePath::U8,
            TermLiteral::U16(_) => PreludeTypePath::U16,
            TermLiteral::U32(_) => PreludeTypePath::U32,
            TermLiteral::U64(_) => PreludeTypePath::U64,
            TermLiteral::U128(_) => PreludeTypePath::U128,
            TermLiteral::USize(_) => PreludeTypePath::USize,
            TermLiteral::R8(_) => PreludeTypePath::R8,
            TermLiteral::R16(_) => PreludeTypePath::R16,
            TermLiteral::R32(_) => PreludeTypePath::R32,
            TermLiteral::R64(_) => PreludeTypePath::R64,
            TermLiteral::R128(_) => PreludeTypePath::R128,
            TermLiteral::RSize(_) => PreludeTypePath::RSize,
            TermLiteral::Nat(_) => PreludeTypePath::Nat,
            TermLiteral::F32(_) => PreludeTypePath::F32,
            TermLiteral::F64(_) => PreludeTypePath::F64,
            TermLiteral::String(_) => PreludeTypePath::StringLiteral,
            TermLiteral::EvalLifetime => PreludeTypePath::Lifetime,
            TermLiteral::StaticLifetime => PreludeTypePath::Lifetime,
        }
    }

    pub fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn TermPreludeDb,
    ) -> std::fmt::Result {
        match self {
            TermLiteral::Unit => f.write_str("unit"),
            TermLiteral::I32(_) => todo!(),
            TermLiteral::I64(_) => todo!(),
            TermLiteral::Nat(_) => todo!(),
            TermLiteral::F32(_) => todo!(),
            TermLiteral::F64(_) => todo!(),
            TermLiteral::Bool(_) => todo!(),
            TermLiteral::String(_) => todo!(),
            TermLiteral::EvalLifetime => f.write_str("'eval"),
            TermLiteral::StaticLifetime => f.write_str("'static"),
            TermLiteral::I8(_) => todo!(),
            TermLiteral::I16(_) => todo!(),
            TermLiteral::I128(_) => todo!(),
            TermLiteral::ISize(_) => todo!(),
            TermLiteral::U8(_) => todo!(),
            TermLiteral::U16(_) => todo!(),
            TermLiteral::U32(_) => todo!(),
            TermLiteral::U64(_) => todo!(),
            TermLiteral::U128(_) => todo!(),
            TermLiteral::USize(_) => todo!(),
            TermLiteral::R8(_) => todo!(),
            TermLiteral::R16(_) => todo!(),
            TermLiteral::R32(_) => todo!(),
            TermLiteral::R64(_) => todo!(),
            TermLiteral::R128(_) => todo!(),
            TermLiteral::RSize(_) => todo!(),
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
