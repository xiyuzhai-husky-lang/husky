use super::*;
use husky_hir_ty::HirConstant;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum LinkageConstant {
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

impl LinkageConstant {
    pub(crate) fn from_hir(hir_constant: HirConstant) -> LinkageConstant {
        match hir_constant {
            HirConstant::Unit(value) => LinkageConstant::Unit(value),
            HirConstant::Bool(value) => LinkageConstant::Bool(value),
            HirConstant::Char(value) => LinkageConstant::Char(value),
            HirConstant::I8(value) => LinkageConstant::I8(value),
            HirConstant::I16(value) => LinkageConstant::I16(value),
            HirConstant::I32(value) => LinkageConstant::I32(value),
            HirConstant::I64(value) => LinkageConstant::I64(value),
            HirConstant::I128(value) => LinkageConstant::I128(value),
            HirConstant::ISize(value) => LinkageConstant::ISize(value),
            HirConstant::U8(value) => LinkageConstant::U8(value),
            HirConstant::U16(value) => LinkageConstant::U16(value),
            HirConstant::U32(value) => LinkageConstant::U32(value),
            HirConstant::U64(value) => LinkageConstant::U64(value),
            HirConstant::U128(value) => LinkageConstant::U128(value),
            HirConstant::USize(value) => LinkageConstant::USize(value),
            HirConstant::R8(value) => LinkageConstant::R8(value),
            HirConstant::R16(value) => LinkageConstant::R16(value),
            HirConstant::R32(value) => LinkageConstant::R32(value),
            HirConstant::R64(value) => LinkageConstant::R64(value),
            HirConstant::R128(value) => LinkageConstant::R128(value),
            HirConstant::RSize(value) => LinkageConstant::RSize(value),
            HirConstant::Symbol(_) => unreachable!(),
        }
    }
}
