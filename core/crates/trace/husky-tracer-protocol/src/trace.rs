mod id;
mod node;
mod stalk;
mod token;

use std::rc::Rc;

pub use id::*;
pub use node::*;
pub use stalk::*;
pub use token::*;

use super::*;

pub type Indent = u8;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceData {
    pub opt_parent_id: Option<TraceId>,
    pub id: TraceId,
    pub kind: TraceKind,
    pub indent: Indent,
    pub lines: Vec<TraceLineData>,
    pub compile_time_version: usize,
    pub can_have_subtraces: bool,
    pub reachable: bool,
}

impl TraceData {
    pub fn associated_trace_ids(&self) -> Vec<TraceId> {
        let mut associated_trace_ids = vec![];
        for line in &self.lines {
            for token in &line.tokens {
                if token.value == "]" || token.value == ")" || token.value == "}" {
                    continue;
                }
                if let Some(associated_trace_id) = token.opt_associated_trace_id {
                    associated_trace_ids.push(associated_trace_id)
                }
            }
        }
        associated_trace_ids
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TraceKind {
    Main,
    FeatureStmt,
    FeatureBranch,
    FeatureExpr,
    FeatureCallArgument,
    FuncStmt,
    ProcStmt,
    ProcBranch,
    LoopFrame,
    EagerExpr,
    EagerCallArgument,
    CallHead,
}

impl TraceKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            TraceKind::Main => "Main",
            TraceKind::FeatureStmt => "FeatureStmt",
            TraceKind::FeatureBranch => "FeatureBranch",
            TraceKind::FeatureExpr => "FeatureExpr",
            TraceKind::FeatureCallArgument => "FeatureCallArgument",
            TraceKind::FuncStmt => "FuncStmt",
            TraceKind::ProcStmt => "ProcStmt",
            TraceKind::ProcBranch => "ProcBranch",
            TraceKind::LoopFrame => "LoopFrame",
            TraceKind::EagerExpr => "EagerExpr",
            TraceKind::EagerCallArgument => "EagerCallArgument",
            TraceKind::CallHead => "CallHead",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceLineData {
    pub indent: Indent,
    pub tokens: Vec<TraceTokenData>,
    pub idx: usize,
}
