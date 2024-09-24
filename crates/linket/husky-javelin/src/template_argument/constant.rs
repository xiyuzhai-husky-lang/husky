use super::*;

use husky_hir_ty::HirCompterm;

#[salsa::derive_debug_with_db]
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
    StaticLifetime,
}

impl JavelinConstant {
    pub(crate) fn from_hir(
        hir_constant: HirCompterm,
        _jav_instantiation: &JavInstantiation,
    ) -> JavelinConstant {
        match hir_constant {
            HirCompterm::Unit(value) => JavelinConstant::Unit(value),
            HirCompterm::Bool(value) => JavelinConstant::Bool(value),
            HirCompterm::Char(value) => JavelinConstant::Char(value),
            HirCompterm::I8(value) => JavelinConstant::I8(value),
            HirCompterm::I16(value) => JavelinConstant::I16(value),
            HirCompterm::I32(value) => JavelinConstant::I32(value),
            HirCompterm::I64(value) => JavelinConstant::I64(value),
            HirCompterm::I128(value) => JavelinConstant::I128(value),
            HirCompterm::ISize(value) => JavelinConstant::ISize(value),
            HirCompterm::U8(value) => JavelinConstant::U8(value),
            HirCompterm::U16(value) => JavelinConstant::U16(value),
            HirCompterm::U32(value) => JavelinConstant::U32(value),
            HirCompterm::U64(value) => JavelinConstant::U64(value),
            HirCompterm::U128(value) => JavelinConstant::U128(value),
            HirCompterm::USize(value) => JavelinConstant::USize(value),
            HirCompterm::R8(value) => JavelinConstant::R8(value),
            HirCompterm::R16(value) => JavelinConstant::R16(value),
            HirCompterm::R32(value) => JavelinConstant::R32(value),
            HirCompterm::R64(value) => JavelinConstant::R64(value),
            HirCompterm::R128(value) => JavelinConstant::R128(value),
            HirCompterm::RSize(value) => JavelinConstant::RSize(value),
            HirCompterm::Symbol(_) => todo!("use javelin instantiation"),
            HirCompterm::TypeVariant(_) => todo!(),
            HirCompterm::StaticLifetime => JavelinConstant::StaticLifetime,
        }
    }
}
