use husky_value::{IsThawedValue, IsValue};
use ordered_float::OrderedFloat;
use std::sync::Arc;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LiteralValue {
    /// unit literal
    Unit(()),
    // todo: /// char literal
    // Char(char),
    /// boolean literal
    Bool(bool),
    /// 8-bit integer literal
    I8(i8),
    /// 16-bit integer literal
    I16(i16),
    /// 32-bit integer literal
    I32(i32),
    /// 64-bit integer literal
    I64(i64),
    /// 128-bit integer literal
    I128(i128),
    /// for cross compilation
    ISize(i64),
    /// 8-bit unsigned integer literal
    U8(u8),
    /// 16-bit unsigned integer liteUal
    U16(u16),
    /// 32-bit unsigned integer liteUal
    U32(u32),
    /// 64-bit unsigned integer liteUal
    U64(u64),
    /// 128-bit unsigned integer liteUal
    U128(u128),
    /// for cross compilation
    USize(u64),
    /// 8-bit raw bit literal
    R8(u8),
    /// 16-bit raw bit literal
    R16(u16),
    /// 32-bit raw bit literal
    R32(u32),
    /// 64-bit raw bit literal
    R64(u64),
    /// 128-bit raw bit literal
    R128(u128),
    /// for cross compilation
    RSize(u64),
    /// 32-bit float literal
    F32(OrderedFloat<f32>),
    /// 64-bit float literal
    F64(OrderedFloat<f64>),
    String(Arc<str>),
}

impl LiteralValue {
    pub fn into_value<Value: IsValue>(&self) -> Value {
        match *self {
            LiteralValue::Unit(()) => ().into(),
            LiteralValue::Bool(b) => b.into(),
            LiteralValue::I8(i) => i.into(),
            LiteralValue::I16(i) => i.into(),
            LiteralValue::I32(i) => i.into(),
            LiteralValue::I64(i) => i.into(),
            LiteralValue::I128(i) => i.into(),
            LiteralValue::ISize(i) => i.into(),
            LiteralValue::U8(u) => u.into(),
            LiteralValue::U16(u) => u.into(),
            LiteralValue::U32(u) => u.into(),
            LiteralValue::U64(u) => u.into(),
            LiteralValue::U128(u) => u.into(),
            LiteralValue::USize(u) => u.into(),
            LiteralValue::R8(r) => Value::from_r8(r),
            LiteralValue::R16(r) => Value::from_r16(r),
            LiteralValue::R32(r) => Value::from_r32(r),
            LiteralValue::R64(r) => Value::from_r64(r),
            LiteralValue::R128(r) => Value::from_r128(r),
            LiteralValue::RSize(r) => Value::from_rsize(r),
            LiteralValue::F32(f) => f.into_inner().into(),
            LiteralValue::F64(f) => f.into_inner().into(),
            LiteralValue::String(ref str_value) => Value::from_str_literal(str_value.clone()),
        }
    }

    pub fn into_thawed_value<ThawedValue: IsThawedValue>(&self) -> ThawedValue {
        match *self {
            LiteralValue::Unit(()) => ().into(),
            LiteralValue::Bool(b) => b.into(),
            LiteralValue::I8(i) => i.into(),
            LiteralValue::I16(i) => i.into(),
            LiteralValue::I32(i) => i.into(),
            LiteralValue::I64(i) => i.into(),
            LiteralValue::I128(i) => i.into(),
            LiteralValue::ISize(i) => i.into(),
            LiteralValue::U8(u) => u.into(),
            LiteralValue::U16(u) => u.into(),
            LiteralValue::U32(u) => u.into(),
            LiteralValue::U64(u) => u.into(),
            LiteralValue::U128(u) => u.into(),
            LiteralValue::USize(u) => u.into(),
            LiteralValue::R8(r) => ThawedValue::from_r8(r),
            LiteralValue::R16(r) => ThawedValue::from_r16(r),
            LiteralValue::R32(r) => ThawedValue::from_r32(r),
            LiteralValue::R64(r) => ThawedValue::from_r64(r),
            LiteralValue::R128(r) => ThawedValue::from_r128(r),
            LiteralValue::RSize(r) => ThawedValue::from_rsize(r),
            LiteralValue::F32(f) => f.into_inner().into(),
            LiteralValue::F64(f) => f.into_inner().into(),
            LiteralValue::String(ref str_value) => ThawedValue::from_str_literal(str_value.clone()),
        }
    }
}
