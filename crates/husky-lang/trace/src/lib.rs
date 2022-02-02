mod figure;
pub mod mock;

use common::*;
use serde::Serialize;

pub use figure::FigureProps;

// ts: { idx: number; parent: number | null; tokens: Token[] }
#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub struct Trace {
    pub id: usize,
    pub parent: Option<usize>,
    pub tokens: Vec<TraceToken>,
}

// ts: { type: string; value: string; spaces_before?: number }
#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub struct TraceToken {
    kind: TraceTokenKind,
    value: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    spaces_before: Option<usize>,
}

// ts: string
#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TraceTokenKind {
    Keyword,
    Ident,
    Special,
    Scope,
    Fade,
}

#[test]
fn test_serialize() {
    p!(serde_json::to_string(&TraceTokenKind::Keyword));
}
