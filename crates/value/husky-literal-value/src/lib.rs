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
        todo!()
    }
}
