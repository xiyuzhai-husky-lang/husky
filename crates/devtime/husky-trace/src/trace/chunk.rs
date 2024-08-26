use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChunkTracePathData {
    biological_parent_path: TracePath,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChunkTraceData {
    source: String,
}
