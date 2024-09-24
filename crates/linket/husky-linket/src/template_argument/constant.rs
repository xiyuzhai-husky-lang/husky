use husky_hir_ty::HirCompterm;
use husky_javelin::template_argument::constant::JavelinConstant;

use crate::instantiation::LinInstantiation;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct LinConstant(pub JavelinConstant);

impl std::ops::Deref for LinConstant {
    type Target = JavelinConstant;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl LinConstant {
    pub(crate) fn from_hir(
        hir_constant: HirCompterm,
        _jav_instantiation: &LinInstantiation,
    ) -> Self {
        Self(match hir_constant {
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
            HirCompterm::TypeVariant(_) => unreachable!(),
            HirCompterm::StaticLifetime => JavelinConstant::StaticLifetime,
        })
    }
}
