use husky_value_interface::IsValue;
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
    String(Arc<String>),
}

impl LiteralValue {
    pub fn into_value<Value: IsValue>(&self) -> Value {
        match *self {
            LiteralValue::Unit(_) => todo!(),
            LiteralValue::Bool(_) => todo!(),
            LiteralValue::I8(_) => todo!(),
            LiteralValue::I16(_) => todo!(),
            LiteralValue::I32(i) => i.into(),
            LiteralValue::I64(_) => todo!(),
            LiteralValue::I128(_) => todo!(),
            LiteralValue::ISize(_) => todo!(),
            LiteralValue::U8(_) => todo!(),
            LiteralValue::U16(_) => todo!(),
            LiteralValue::U32(_) => todo!(),
            LiteralValue::U64(_) => todo!(),
            LiteralValue::U128(_) => todo!(),
            LiteralValue::USize(_) => todo!(),
            LiteralValue::R8(_) => todo!(),
            LiteralValue::R16(_) => todo!(),
            LiteralValue::R32(_) => todo!(),
            LiteralValue::R64(_) => todo!(),
            LiteralValue::R128(_) => todo!(),
            LiteralValue::RSize(_) => todo!(),
            LiteralValue::F32(_) => todo!(),
            LiteralValue::F64(_) => todo!(),
            LiteralValue::String(_) => todo!(),
        }
    }
}
