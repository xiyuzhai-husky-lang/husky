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
pub struct TraceRawData {
    pub opt_parent_id: Option<TraceId>,
    pub id: TraceId,
    pub kind: TraceKind,
    pub indent: Indent,
    pub lines: Vec<TraceLineRawData>,
    pub compile_time_version: usize,
    pub can_have_subtraces: bool,
    pub reachable: bool,
}

#[derive(Debug, Clone)]
pub struct TraceData {
    pub opt_parent_id: Option<TraceId>,
    pub id: TraceId,
    pub kind: TraceKind,
    pub indent: Indent,
    pub lines: Rc<Vec<TraceLineData>>,
    pub compile_time_version: usize,
    pub can_have_subtraces: bool,
    pub reachable: bool,
}

impl From<TraceRawData> for TraceData {
    fn from(raw_data: TraceRawData) -> Self {
        Self {
            opt_parent_id: raw_data.opt_parent_id,
            id: raw_data.id,
            kind: raw_data.kind,
            indent: raw_data.indent,
            lines: Rc::new(raw_data.lines.into_iter().map(|line| line.into()).collect()),
            compile_time_version: raw_data.compile_time_version,
            can_have_subtraces: raw_data.can_have_subtraces,
            reachable: raw_data.reachable,
        }
    }
}

impl TraceRawData {
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
    FeatureCallInput,
    FuncStmt,
    ProcStmt,
    ProcBranch,
    LoopFrame,
    EagerExpr,
    CallHead,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceLineRawData {
    pub indent: Indent,
    pub tokens: Vec<TraceTokenData>,
    pub idx: usize,
}

#[derive(Debug, Clone)]
pub struct TraceLineData {
    pub indent: Indent,
    pub tokens: Vec<Rc<TraceTokenData>>,
    pub idx: usize,
}

impl From<TraceLineRawData> for TraceLineData {
    fn from(raw_data: TraceLineRawData) -> Self {
        Self {
            indent: raw_data.indent,
            tokens: raw_data
                .tokens
                .into_iter()
                .map(|token_data| Rc::new(token_data))
                .collect(),
            idx: raw_data.idx,
        }
    }
}
