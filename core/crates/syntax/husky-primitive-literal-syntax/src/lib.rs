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
        match self {
            PrimitiveLiteralData::Void => "void".to_string(),
            PrimitiveLiteralData::Integer(i) => format!("{i}"),
            PrimitiveLiteralData::I32(i) => format!("{i}"),
            PrimitiveLiteralData::I64(i) => format!("{i}"),
            PrimitiveLiteralData::Float(f) => format!("{f}"),
            PrimitiveLiteralData::F32(f) => format!("{f}"),
            PrimitiveLiteralData::F64(f) => format!("{f}"),
            PrimitiveLiteralData::Bits(_) => todo!(),
            PrimitiveLiteralData::B32(_) => todo!(),
            PrimitiveLiteralData::B64(_) => todo!(),
            PrimitiveLiteralData::Bool(_) => todo!(),
        }
    }
}
