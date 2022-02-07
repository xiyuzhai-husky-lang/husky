mod eval;
mod figure;
mod kind;
pub mod mock;
#[cfg(test)]
mod tests;
mod token;

pub use eval::eval_block_traces;
pub use figure::FigureProps;
pub use kind::TraceKind;
pub use token::{TraceToken, TraceTokenKind};

use std::{
    borrow::Cow,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use common::*;
use feature::FeatureBlock;
use file::FilePtr;
use serde::{ser::SerializeStruct, Serialize};

use token::*;

// ts: { idx: number; parent: number | null; tokens: Token[] }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Trace {
    pub id: usize,
    pub parent: Option<usize>,
    pub kind: TraceKind,
}

impl Serialize for Trace {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Trace", 3)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("parent", &self.parent)?;
        state.serialize_field("tokens", &self.tokens())?;
        state.end()
    }
}

static NEXT_TRACE_ID: AtomicUsize = AtomicUsize::new(0);
static NEXT_TRACE_ID_ORDERING: Ordering = Ordering::SeqCst;

impl Trace {
    fn new(parent: Option<usize>, kind: TraceKind) -> Arc<Self> {
        let id = NEXT_TRACE_ID.load(NEXT_TRACE_ID_ORDERING);
        NEXT_TRACE_ID.store(id, NEXT_TRACE_ID_ORDERING);
        Arc::new(Self { id, parent, kind })
    }

    fn mock(id: usize, parent: Option<usize>, tokens: Vec<TraceToken>) -> Arc<Self> {
        NEXT_TRACE_ID.store(id, NEXT_TRACE_ID_ORDERING);
        Arc::new(Self {
            id,
            parent,
            kind: TraceKind::Mock { tokens },
        })
    }

    pub fn main(main_file: FilePtr, feature_block: Arc<FeatureBlock>) -> Arc<Self> {
        Self::new(
            None,
            TraceKind::Main {
                main_file,
                feature_block,
            },
        )
    }

    pub fn tokens(&self) -> Cow<[TraceToken]> {
        match self.kind {
            TraceKind::Mock { ref tokens } => tokens.into(),
            TraceKind::Main { .. } => Cow::Borrowed(&[TraceToken {
                kind: TraceTokenKind::Keyword,
                value: "main",
                spaces_before: None,
            }]),
            TraceKind::Stmt(ref stmt) => match stmt.kind {
                feature::FeatureStmtKind::Init { varname, ref value } => {
                    let mut tokens = vec![];
                    tokens.push(ident!(varname.0));
                    tokens.push(special!("="));
                    tokens.push(special!("todo"));
                    tokens.into()
                }
                feature::FeatureStmtKind::Assert { ref condition } => {
                    let mut tokens = vec![];
                    tokens.push(keyword!("assert"));
                    tokens.push(special!("todo"));
                    tokens.into()
                }
                feature::FeatureStmtKind::Return { ref result } => {
                    let mut tokens = vec![];
                    tokens.push(keyword!("return todo"));
                    tokens.into()
                }
            },
            TraceKind::Expr(_) => todo!(),
        }
    }
}
