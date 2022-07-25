use ordered_float::OrderedFloat;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PrimitiveLiteralData {
    Void,
    Integer(i64),
    I32(i32),
    I64(i64),
    Float(OrderedFloat<f64>),
    F32(OrderedFloat<f32>),
    F64(OrderedFloat<f64>),
    Bits(u64),
    B32(u32),
    B64(u64),
    Bool(bool),
}

impl Into<String> for PrimitiveLiteralData {
    fn into(self) -> String {
        todo!()
    }
}
