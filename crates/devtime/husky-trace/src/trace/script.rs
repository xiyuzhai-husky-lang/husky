use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ScriptTracePathData {
    biological_parent_path: TracePath,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ScriptTraceData {
    source: String,
}
