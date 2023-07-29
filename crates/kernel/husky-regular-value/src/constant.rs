use crate::*;
use husky_hir_constant::{db::HirConstantDb, HirConstant};

/// precompiled to save conversion
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct __RegularValueConstant([u64; REGULAR_VALUE_SIZE_OVER_I64]);

impl __RegularValueConstant {
    // change to HirConstant instead
    pub fn new(constant: HirConstant, db: &dyn HirConstantDb) -> Self {
        let value: __RegularValue = match constant {
            HirConstant::Unit(()) => __RegularValue::Unit(()),
            HirConstant::Bool(v) => __RegularValue::Bool(v),
            HirConstant::Char(v) => __RegularValue::Char(v),
            HirConstant::I8(v) => __RegularValue::I8(v),
            HirConstant::I16(v) => __RegularValue::I16(v),
            HirConstant::I32(v) => __RegularValue::I32(v),
            HirConstant::I64(v) => __RegularValue::I64(v),
            HirConstant::I128(v) => __RegularValue::I128(v),
            HirConstant::ISize(v) => __RegularValue::ISize(v),
            HirConstant::U8(v) => __RegularValue::U8(v),
            HirConstant::U16(v) => __RegularValue::U16(v),
            HirConstant::U32(v) => __RegularValue::U32(v),
            HirConstant::U64(v) => __RegularValue::U64(v),
            HirConstant::U128(v) => __RegularValue::U128(v),
            HirConstant::USize(v) => __RegularValue::USize(v),
            HirConstant::R8(v) => __RegularValue::R8(v),
            HirConstant::R16(v) => __RegularValue::R16(v),
            HirConstant::R32(v) => __RegularValue::R32(v),
            HirConstant::R64(v) => __RegularValue::R64(v),
            HirConstant::R128(v) => __RegularValue::R128(v),
            HirConstant::RSize(v) => __RegularValue::RSize(v),
            HirConstant::F32(v) => __RegularValue::F32(v.into()),
            HirConstant::F64(v) => __RegularValue::F64(v.into()),
            HirConstant::StringLiteral(v) => __RegularValue::StringLiteral(v.into()),
        };
        Self(unsafe { std::mem::transmute(value) })
    }
}

impl From<__RegularValueConstant> for __RegularValue {
    fn from(constant: __RegularValueConstant) -> Self {
        unsafe { std::mem::transmute(constant.0) }
    }
}
