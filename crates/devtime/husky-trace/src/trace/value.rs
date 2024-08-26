use super::TracePath;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ValueTracePathData {
    biological_parent_path: TracePath,
    variant: ValueTraceAccessData,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ValueTraceAccessData {
    StructField,
    EnumVariantField,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ValueTraceData {}
