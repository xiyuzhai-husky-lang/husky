use crate::*;
use husky_hir_prelude::{db::HirPreludeDb, HirLiteral};

/// precompiled to save conversion
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct __RegularValueConstant([u64; REGULAR_VALUE_SIZE_OVER_I64]);

impl __RegularValueConstant {
    // change to HirConstant instead
    pub fn new(constant: HirLiteral, db: &dyn HirPreludeDb) -> Self {
        let value: RegularValue = match constant {
            HirLiteral::Unit(()) => RegularValue::Unit(()),
            HirLiteral::Bool(v) => RegularValue::Bool(v),
            HirLiteral::Char(v) => RegularValue::Char(v),
            HirLiteral::I8(v) => RegularValue::I8(v),
            HirLiteral::I16(v) => RegularValue::I16(v),
            HirLiteral::I32(v) => RegularValue::I32(v),
            HirLiteral::I64(v) => RegularValue::I64(v),
            HirLiteral::I128(v) => RegularValue::I128(v),
            HirLiteral::ISize(v) => RegularValue::ISize(v),
            HirLiteral::U8(v) => RegularValue::U8(v),
            HirLiteral::U16(v) => RegularValue::U16(v),
            HirLiteral::U32(v) => RegularValue::U32(v),
            HirLiteral::U64(v) => RegularValue::U64(v),
            HirLiteral::U128(v) => RegularValue::U128(v),
            HirLiteral::USize(v) => RegularValue::USize(v),
            HirLiteral::R8(v) => RegularValue::R8(v),
            HirLiteral::R16(v) => RegularValue::R16(v),
            HirLiteral::R32(v) => RegularValue::R32(v),
            HirLiteral::R64(v) => RegularValue::R64(v),
            HirLiteral::R128(v) => RegularValue::R128(v),
            HirLiteral::RSize(v) => RegularValue::RSize(v),
            HirLiteral::F32(v) => RegularValue::F32(v.into()),
            HirLiteral::F64(v) => RegularValue::F64(v.into()),
            HirLiteral::StringLiteral(v) => RegularValue::StringLiteral(v.into()),
        };
        Self(unsafe { std::mem::transmute(value) })
    }
}

impl From<__RegularValueConstant> for RegularValue {
    fn from(constant: __RegularValueConstant) -> Self {
        unsafe { std::mem::transmute(constant.0) }
    }
}
