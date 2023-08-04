use crate::*;
use ordered_float::OrderedFloat;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = TermPreludeDb, jar = TermPreludeJar)]
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
    I64(TermI64Literal),
    /// 128-bit integer literal
    I128(TermI128Literal),
    /// for cross compilation
    ISize(TermISizeLiteral),
    /// 8-bit unsigned integer literal
    U8(u8),
    /// 16-bit unsigned integer liteUal
    U16(u16),
    /// 32-bit unsigned integer liteUal
    U32(u32),
    /// 64-bit unsigned integer liteUal
    U64(TermU64Literal),
    /// 128-bit unsigned integer liteUal
    U128(TermU128Literal),
    /// for cross compilation
    USize(TermUSizeLiteral),
    /// 8-bit raw bit literal
    R8(u8),
    /// 16-bit raw bit literal
    R16(u16),
    /// 32-bit raw bit literal
    R32(u32),
    /// 64-bit raw bit literal
    R64(TermI64Literal),
    /// 128-bit raw bit literal
    R128(TermR128Literal),
    /// for cross compilation
    RSize(TermRSizeLiteral),
    /// natural number literal
    Nat(TermNatLiteral),
    /// 32-bit float literal
    F32(OrderedFloat<f32>),
    /// 64-bit float literal
    F64(TermF64Literal),
    /// string literal
    String(StringLiteral),
    /// static lifetime
    StaticLifetime,
}

#[salsa::tracked(db = TermPreludeDb, jar = TermPreludeJar)]
pub struct StringLiteral {
    data: String,
}

#[test]
fn term_literal_size_works() {
    assert_eq!(
        std::mem::size_of::<TermLiteral>(),
        std::mem::size_of::<u64>()
    )
}

impl TermLiteral {
    #[inline(always)]
    pub(crate) fn from_declarative(_db: &dyn TermPreludeDb, _valid_term: TermLiteral) -> Self {
        todo!()
    }

    pub fn ty(self) -> PreludeTypePath {
        match self {
            TermLiteral::Unit => PreludeBasicTypePath::Unit.into(),
            TermLiteral::Unit => PreludeBasicTypePath::Never.into(),
            TermLiteral::Bool(_) => PreludeBasicTypePath::Bool.into(),
            TermLiteral::I8(_) => PreludeIntTypePath::I8.into(),
            TermLiteral::I16(_) => PreludeIntTypePath::I16.into(),
            TermLiteral::I32(_) => PreludeIntTypePath::I32.into(),
            TermLiteral::I64(_) => PreludeIntTypePath::I64.into(),
            TermLiteral::I128(_) => PreludeIntTypePath::I128.into(),
            TermLiteral::ISize(_) => PreludeIntTypePath::ISize.into(),
            TermLiteral::U8(_) => PreludeIntTypePath::U8.into(),
            TermLiteral::U16(_) => PreludeIntTypePath::U16.into(),
            TermLiteral::U32(_) => PreludeIntTypePath::U32.into(),
            TermLiteral::U64(_) => PreludeIntTypePath::U64.into(),
            TermLiteral::U128(_) => PreludeIntTypePath::U128.into(),
            TermLiteral::USize(_) => PreludeIntTypePath::USize.into(),
            TermLiteral::R8(_) => PreludeIntTypePath::R8.into(),
            TermLiteral::R16(_) => PreludeIntTypePath::R16.into(),
            TermLiteral::R32(_) => PreludeIntTypePath::R32.into(),
            TermLiteral::R64(_) => PreludeIntTypePath::R64.into(),
            TermLiteral::R128(_) => PreludeIntTypePath::R128.into(),
            TermLiteral::RSize(_) => PreludeIntTypePath::RSize.into(),
            TermLiteral::Nat(_) => todo!(),
            // PreludeIntegerTypePath::Nat.into(),
            TermLiteral::F32(_) => PreludeFloatTypePath::F32.into(),
            TermLiteral::F64(_) => PreludeFloatTypePath::F64.into(),
            TermLiteral::String(_) => PreludeTypePath::StringLiteral,
            TermLiteral::StaticLifetime => PreludeTypePath::Lifetime,
        }
    }

    pub fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn TermPreludeDb,
    ) -> std::fmt::Result {
        use std::fmt::Display;
        match self {
            TermLiteral::Unit => f.write_str("unit"),
            TermLiteral::I32(_) => todo!(),
            TermLiteral::I64(_) => todo!(),
            TermLiteral::Nat(_) => todo!(),
            TermLiteral::F32(_) => todo!(),
            TermLiteral::F64(_) => todo!(),
            TermLiteral::Bool(_) => todo!(),
            TermLiteral::String(_) => todo!(),
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
            TermLiteral::USize(lit) => lit.value(db).fmt(f),
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
pub struct TermI64Literal {
    pub value: i64,
}

/// allowing representing very large number
#[salsa::interned(db = TermPreludeDb, jar = TermPreludeJar)]
pub struct TermI128Literal {
    pub value: i128,
}

/// allowing representing very large number
#[salsa::interned(db = TermPreludeDb, jar = TermPreludeJar)]
pub struct TermI256Literal {
    pub value: [i128; 2],
}

/// allowing representing very large number
#[salsa::interned(db = TermPreludeDb, jar = TermPreludeJar)]
pub struct TermISizeLiteral {
    pub value: i64,
}

/// allowing representing very large number
#[salsa::interned(db = TermPreludeDb, jar = TermPreludeJar)]
pub struct TermU64Literal {
    pub value: u64,
}

/// allowing representing very large number
#[salsa::interned(db = TermPreludeDb, jar = TermPreludeJar)]
pub struct TermU128Literal {
    pub value: u128,
}

/// allowing representing very large number
#[salsa::interned(db = TermPreludeDb, jar = TermPreludeJar)]
pub struct TermU256Literal {
    pub value: [u128; 2],
}

/// allowing representing very large number
#[salsa::interned(db = TermPreludeDb, jar = TermPreludeJar)]
pub struct TermUSizeLiteral {
    pub value: u64,
}

/// allowing representing very large number
#[salsa::interned(db = TermPreludeDb, jar = TermPreludeJar)]
pub struct TermR64Literal {
    pub value: u64,
}

/// allowing representing very large number
#[salsa::interned(db = TermPreludeDb, jar = TermPreludeJar)]
pub struct TermR128Literal {
    pub value: u128,
}

/// allowing representing very large number
#[salsa::interned(db = TermPreludeDb, jar = TermPreludeJar)]
pub struct TermR256Literal {
    pub value: [u128; 2],
}

/// allowing representing very large number
#[salsa::interned(db = TermPreludeDb, jar = TermPreludeJar)]
pub struct TermRSizeLiteral {
    pub value: u64,
}

/// allowing representing very large number
#[salsa::interned(db = TermPreludeDb, jar = TermPreludeJar)]
pub struct TermF64Literal {
    pub value: OrderedFloat<f64>,
}

/// allowing representing very large number
#[salsa::interned(db = TermPreludeDb, jar = TermPreludeJar)]
pub struct TermNatLiteral {
    pub bits: Vec<usize>,
}

impl<Db: TermPreludeDb + ?Sized> salsa::DisplayWithDb<Db> for TermLiteral {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        _level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<TermPreludeJar>>::as_jar_db(db);
        self.show_with_db_fmt(f, db)
    }
}
