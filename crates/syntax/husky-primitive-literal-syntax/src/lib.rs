use ordered_float::OrderedFloat;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum RawLiteralData {
    Unit,
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

impl RawLiteralData {
    pub fn negative(self) -> Option<RawLiteralData> {
        match self {
            RawLiteralData::Unit => None,
            RawLiteralData::Integer(i) => Some(RawLiteralData::Integer(-i)),
            RawLiteralData::I32(i) => Some(RawLiteralData::I32(-i)),
            RawLiteralData::I64(i) => Some(RawLiteralData::I64(-i)),
            RawLiteralData::Float(f) => Some(RawLiteralData::Float(-f)),
            RawLiteralData::F32(f) => Some(RawLiteralData::F32(-f)),
            RawLiteralData::F64(f) => Some(RawLiteralData::F64(-f)),
            RawLiteralData::Bits(_) => None,
            RawLiteralData::B32(_) => None,
            RawLiteralData::B64(_) => None,
            RawLiteralData::Bool(_) => None,
        }
    }
}

impl std::fmt::Display for RawLiteralData {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RawLiteralData::Unit => "void".fmt(formatter),
            RawLiteralData::Integer(i) => i.fmt(formatter),
            RawLiteralData::I32(i) => i.fmt(formatter),
            RawLiteralData::I64(i) => i.fmt(formatter),
            RawLiteralData::Float(f) => f.fmt(formatter),
            RawLiteralData::F32(f) => f.fmt(formatter),
            RawLiteralData::F64(f) => f.fmt(formatter),
            RawLiteralData::Bits(b) => b.fmt(formatter),
            RawLiteralData::B32(b) => b.fmt(formatter),
            RawLiteralData::B64(b) => b.fmt(formatter),
            RawLiteralData::Bool(b) => b.fmt(formatter),
        }
    }
}

impl Into<String> for RawLiteralData {
    fn into(self) -> String {
        match self {
            RawLiteralData::Unit => "void".to_string(),
            RawLiteralData::Integer(i) => format!("{i}"),
            RawLiteralData::I32(i) => format!("{i}i32"),
            RawLiteralData::I64(i) => format!("{i}i64"),
            RawLiteralData::Float(f) => format!("{f}"),
            RawLiteralData::F32(f) => format!("{f}f32"),
            RawLiteralData::F64(f) => format!("{f}f64"),
            RawLiteralData::Bits(b) => format!("{b}"),
            RawLiteralData::B32(b) => format!("{b}u32"),
            RawLiteralData::B64(b) => format!("{b}u64"),
            RawLiteralData::Bool(b) => format!("{b}"),
        }
    }
}
