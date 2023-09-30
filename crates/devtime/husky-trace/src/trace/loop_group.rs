use husky_trace_path::TracePath;

use super::*;

#[salsa::tracked(db = TraceDb, jar = TraceJar)]
pub struct LoopGroupTrace {
    #[id]
    pub path: TracePath,
    pub biological_parent: LoopGroupTraceBiologicalParent,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LoopGroupTraceBiologicalParent {}
