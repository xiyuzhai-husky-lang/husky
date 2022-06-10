mod id;
mod stalk;
mod token;

pub use id::*;
pub use stalk::*;
pub use token::*;

use super::*;

pub type Indent = u8;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceProps {
    pub opt_parent: Option<TraceId>,
    pub id: TraceId,
    pub kind: TraceKind,
    pub indent: Indent,
    pub lines: Vec<LineProps>,
    pub compile_time_version: usize,
    pub has_subtraces: bool,
    pub reachable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TraceKind {
    Main,
    FeatureStmt,
    FeatureBranch,
    FeatureExpr,
    FeatureCallInput,
    FuncStmt,
    ProcStmt,
    ProcBranch,
    LoopFrame,
    EagerExpr,
    CallHead,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineProps {
    pub indent: Indent,
    pub tokens: Vec<TraceTokenProps>,
    pub idx: usize,
}
