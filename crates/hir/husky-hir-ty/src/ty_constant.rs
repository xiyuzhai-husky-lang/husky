use crate::*;
use husky_entity_path::path::ty_variant::TypeVariantPath;
use husky_term_prelude::literal::Literal;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum HirCompterm {
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
    Symbol(HirComptermTemplateVariable),
    // todo: instantiation and fields
    TypeVariant(TypeVariantPath),
    StaticLifetime,
}

impl From<TypeVariantPath> for HirCompterm {
    fn from(path: TypeVariantPath) -> Self {
        HirCompterm::TypeVariant(path)
    }
}

impl HirCompterm {
    pub fn from_term(term_lit: Literal, db: &::salsa::Db) -> Self {
        match term_lit {
            Literal::Unit(()) => todo!(),
            Literal::Bool(_) => todo!(),
            Literal::I8(_) => todo!(),
            Literal::I16(_) => todo!(),
            Literal::I32(_) => todo!(),
            Literal::I64(_) => todo!(),
            Literal::I128(_) => todo!(),
            Literal::ISize(_) => todo!(),
            Literal::U8(_) => todo!(),
            Literal::U16(_) => todo!(),
            Literal::U32(_) => todo!(),
            Literal::U64(_) => todo!(),
            Literal::U128(_) => todo!(),
            Literal::USize(val) => HirCompterm::USize(val.value(db) as usize), // ad hoc
            Literal::R8(_) => todo!(),
            Literal::R16(_) => todo!(),
            Literal::R32(_) => todo!(),
            Literal::R64(_) => todo!(),
            Literal::R128(_) => todo!(),
            Literal::RSize(_) => todo!(),
            Literal::Nat(_) => todo!(),
            Literal::F32(_) => todo!(),
            Literal::F64(_) => todo!(),
            Literal::String(_) => todo!(),
            Literal::StaticLifetime => HirCompterm::StaticLifetime,
        }
    }
}

impl From<HirComptermTemplateVariable> for HirCompterm {
    fn from(symbol: HirComptermTemplateVariable) -> Self {
        HirCompterm::Symbol(symbol)
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
