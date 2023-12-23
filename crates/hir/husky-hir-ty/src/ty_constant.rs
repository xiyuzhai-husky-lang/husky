use husky_term_prelude::TermLiteral;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum HirConstant {
    Unit(()),
    Bool(bool),
    Char(char),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    ISize(isize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    // we don't use usize here due to cross-compilation
    USize(usize),
    R8(u8),
    R16(u16),
    R32(u32),
    R64(u64),
    R128(u128),
    RSize(usize),
    // todo: should we add more types here?
    // Rust only allows for these things
    Symbol(HirConstSymbol),
    // todo: instantiation and fields
    TypeVariant(TypeVariantPath),
}

impl From<TypeVariantPath> for HirConstant {
    fn from(path: TypeVariantPath) -> Self {
        HirConstant::TypeVariant(path)
    }
}

impl HirConstant {
    pub fn from_term(term_lit: TermLiteral, db: &::salsa::Db) -> Self {
        match term_lit {
            TermLiteral::Unit(()) => todo!(),
            TermLiteral::Bool(_) => todo!(),
            TermLiteral::I8(_) => todo!(),
            TermLiteral::I16(_) => todo!(),
            TermLiteral::I32(_) => todo!(),
            TermLiteral::I64(_) => todo!(),
            TermLiteral::I128(_) => todo!(),
            TermLiteral::ISize(_) => todo!(),
            TermLiteral::U8(_) => todo!(),
            TermLiteral::U16(_) => todo!(),
            TermLiteral::U32(_) => todo!(),
            TermLiteral::U64(_) => todo!(),
            TermLiteral::U128(_) => todo!(),
            TermLiteral::USize(val) => HirConstant::USize(val.value(db) as usize), // ad hoc
            TermLiteral::R8(_) => todo!(),
            TermLiteral::R16(_) => todo!(),
            TermLiteral::R32(_) => todo!(),
            TermLiteral::R64(_) => todo!(),
            TermLiteral::R128(_) => todo!(),
            TermLiteral::RSize(_) => todo!(),
            TermLiteral::Nat(_) => todo!(),
            TermLiteral::F32(_) => todo!(),
            TermLiteral::F64(_) => todo!(),
            TermLiteral::String(_) => todo!(),
            TermLiteral::StaticLifetime => todo!(),
        }
    }
}

impl From<HirConstSymbol> for HirConstant {
    fn from(symbol: HirConstSymbol) -> Self {
        HirConstant::Symbol(symbol)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum HirConstantLiteral {
    Unit(()),
    Bool(bool),
    Char(char),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    ISize(isize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    // we don't use usize here due to cross-compilation
    USize(usize),
    R8(u8),
    R16(u16),
    R32(u32),
    R64(u64),
    R128(u128),
    RSize(usize),
}
