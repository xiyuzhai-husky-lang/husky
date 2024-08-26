use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReplTracePathData {
    biological_parent_path: TracePath,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReplTraceData {
    source: String,
}
