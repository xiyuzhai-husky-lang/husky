use ordered_float::OrderedFloat;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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

impl std::fmt::Display for PrimitiveLiteralData {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrimitiveLiteralData::Void => "void".fmt(formatter),
            PrimitiveLiteralData::Integer(i) => i.fmt(formatter),
            PrimitiveLiteralData::I32(i) => i.fmt(formatter),
            PrimitiveLiteralData::I64(i) => i.fmt(formatter),
            PrimitiveLiteralData::Float(f) => f.fmt(formatter),
            PrimitiveLiteralData::F32(f) => f.fmt(formatter),
            PrimitiveLiteralData::F64(f) => f.fmt(formatter),
            PrimitiveLiteralData::Bits(b) => b.fmt(formatter),
            PrimitiveLiteralData::B32(b) => b.fmt(formatter),
            PrimitiveLiteralData::B64(b) => b.fmt(formatter),
            PrimitiveLiteralData::Bool(b) => b.fmt(formatter),
        }
    }
}

impl Into<String> for PrimitiveLiteralData {
    fn into(self) -> String {
        match self {
            PrimitiveLiteralData::Void => "void".to_string(),
            PrimitiveLiteralData::Integer(i) => format!("{i}"),
            PrimitiveLiteralData::I32(i) => format!("{i}i32"),
            PrimitiveLiteralData::I64(i) => format!("{i}i64"),
            PrimitiveLiteralData::Float(f) => format!("{f}"),
            PrimitiveLiteralData::F32(f) => format!("{f}f32"),
            PrimitiveLiteralData::F64(f) => format!("{f}f64"),
            PrimitiveLiteralData::Bits(b) => format!("{b}"),
            PrimitiveLiteralData::B32(b) => format!("{b}u32"),
            PrimitiveLiteralData::B64(b) => format!("{b}u64"),
            PrimitiveLiteralData::Bool(b) => format!("{b}"),
        }
    }
}
