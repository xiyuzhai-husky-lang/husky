use super::*;
use husky_hir_ty::HirConstant;

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum JavelinConstant {
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

impl JavelinConstant {
    pub(crate) fn from_hir(
        hir_constant: HirConstant,
        javelin_instantiation: Option<&JavelinInstantiation>,
    ) -> JavelinConstant {
        match hir_constant {
            HirConstant::Unit(value) => JavelinConstant::Unit(value),
            HirConstant::Bool(value) => JavelinConstant::Bool(value),
            HirConstant::Char(value) => JavelinConstant::Char(value),
            HirConstant::I8(value) => JavelinConstant::I8(value),
            HirConstant::I16(value) => JavelinConstant::I16(value),
            HirConstant::I32(value) => JavelinConstant::I32(value),
            HirConstant::I64(value) => JavelinConstant::I64(value),
            HirConstant::I128(value) => JavelinConstant::I128(value),
            HirConstant::ISize(value) => JavelinConstant::ISize(value),
            HirConstant::U8(value) => JavelinConstant::U8(value),
            HirConstant::U16(value) => JavelinConstant::U16(value),
            HirConstant::U32(value) => JavelinConstant::U32(value),
            HirConstant::U64(value) => JavelinConstant::U64(value),
            HirConstant::U128(value) => JavelinConstant::U128(value),
            HirConstant::USize(value) => JavelinConstant::USize(value),
            HirConstant::R8(value) => JavelinConstant::R8(value),
            HirConstant::R16(value) => JavelinConstant::R16(value),
            HirConstant::R32(value) => JavelinConstant::R32(value),
            HirConstant::R64(value) => JavelinConstant::R64(value),
            HirConstant::R128(value) => JavelinConstant::R128(value),
            HirConstant::RSize(value) => JavelinConstant::RSize(value),
            HirConstant::Symbol(_) => todo!("use javelin instantiation"),
        }
    }
}
