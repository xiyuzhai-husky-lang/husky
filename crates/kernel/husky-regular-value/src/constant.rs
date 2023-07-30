use crate::*;
use husky_hir_prelude::{db::HirPreludeDb, HirLiteral};

/// precompiled to save conversion
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct __RegularValueConstant([u64; REGULAR_VALUE_SIZE_OVER_I64]);

impl __RegularValueConstant {
    // change to HirConstant instead
    pub fn new(constant: HirLiteral, db: &dyn HirPreludeDb) -> Self {
        let value: __RegularValue = match constant {
            HirLiteral::Unit(()) => __RegularValue::Unit(()),
            HirLiteral::Bool(v) => __RegularValue::Bool(v),
            HirLiteral::Char(v) => __RegularValue::Char(v),
            HirLiteral::I8(v) => __RegularValue::I8(v),
            HirLiteral::I16(v) => __RegularValue::I16(v),
            HirLiteral::I32(v) => __RegularValue::I32(v),
            HirLiteral::I64(v) => __RegularValue::I64(v),
            HirLiteral::I128(v) => __RegularValue::I128(v),
            HirLiteral::ISize(v) => __RegularValue::ISize(v),
            HirLiteral::U8(v) => __RegularValue::U8(v),
            HirLiteral::U16(v) => __RegularValue::U16(v),
            HirLiteral::U32(v) => __RegularValue::U32(v),
            HirLiteral::U64(v) => __RegularValue::U64(v),
            HirLiteral::U128(v) => __RegularValue::U128(v),
            HirLiteral::USize(v) => __RegularValue::USize(v),
            HirLiteral::R8(v) => __RegularValue::R8(v),
            HirLiteral::R16(v) => __RegularValue::R16(v),
            HirLiteral::R32(v) => __RegularValue::R32(v),
            HirLiteral::R64(v) => __RegularValue::R64(v),
            HirLiteral::R128(v) => __RegularValue::R128(v),
            HirLiteral::RSize(v) => __RegularValue::RSize(v),
            HirLiteral::F32(v) => __RegularValue::F32(v.into()),
            HirLiteral::F64(v) => __RegularValue::F64(v.into()),
            HirLiteral::StringLiteral(v) => __RegularValue::StringLiteral(v.into()),
        };
        Self(unsafe { std::mem::transmute(value) })
    }
}

impl From<__RegularValueConstant> for __RegularValue {
    fn from(constant: __RegularValueConstant) -> Self {
        unsafe { std::mem::transmute(constant.0) }
    }
}
