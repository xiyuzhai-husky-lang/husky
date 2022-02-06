mod figure;
pub mod mock;

use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Mutex,
};

use common::*;
use serde::Serialize;

pub use figure::FigureProps;

// ts: { idx: number; parent: number | null; tokens: Token[] }
#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub struct Trace {
    id: usize,
    parent: Option<usize>,
    tokens: Vec<TraceToken>,
}

static NEXT_TRACE_ID: AtomicUsize = AtomicUsize::new(0);
static NEXT_TRACE_ID_ORDERING: Ordering = Ordering::SeqCst;

impl Trace {
    fn new(parent: Option<usize>, tokens: Vec<TraceToken>) -> Arc<Self> {
        let id = NEXT_TRACE_ID.load(NEXT_TRACE_ID_ORDERING);
        NEXT_TRACE_ID.store(id, NEXT_TRACE_ID_ORDERING);
        Arc::new(Self { id, parent, tokens })
    }

    pub fn main() -> Arc<Self> {
        let tokens = vec![TraceToken {
            kind: TraceTokenKind::Keyword,
            value: "main",
            spaces_before: None,
        }];
        Self::new(None, tokens)
    }

    pub fn id(&self) -> usize {
        self.id
    }
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
