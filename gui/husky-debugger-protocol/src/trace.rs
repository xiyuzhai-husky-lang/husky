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
    pub opt_parent_id: Option<TraceId>,
    pub id: TraceId,
    pub kind: TraceKind,
    pub indent: Indent,
    pub lines: Vec<LineProps>,
    pub compile_time_version: usize,
    pub has_subtraces: bool,
    pub reachable: bool,
}

impl TraceProps {
    pub fn collect_associated_traces(&self, associated_traces: &mut Vec<TraceId>) {
        for line in &self.lines {
            for token in &line.tokens {
                if token.value == "]" || token.value == ")" || token.value == "}" {
                    continue;
                }
                if let Some(associated_trace) = &token.opt_associated_trace_id {
                    associated_traces.push(associated_trace.clone())
                }
            }
        }
    }
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
