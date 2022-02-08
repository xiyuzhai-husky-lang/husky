mod eval;
mod figure;
mod kind;
pub mod mock;
#[cfg(test)]
mod tests;
mod token;

pub use eval::{eval_block_subtraces, eval_feature_expr_subtraces, eval_feature_stmt_subtraces};
use eval::{eval_feature_expr_trace_tokens, eval_feature_stmt_trace_tokens};
pub use figure::FigureProps;
use fold::Indent;
pub use kind::TraceKind;
pub use token::{TokenProps, TraceTokenKind};

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
    parent: Option<usize>,
    pub id: usize,
    pub indent: fold::Indent,
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
        state.serialize_field("indent", &self.indent)?;
        state.serialize_field("tokens", &self.tokens())?;
        state.end()
    }
}

static NEXT_TRACE_ID: AtomicUsize = AtomicUsize::new(0);
static NEXT_TRACE_ID_ORDERING: Ordering = Ordering::SeqCst;

impl Trace {
    fn new(parent: Option<usize>, indent: Indent, kind: TraceKind) -> Arc<Self> {
        let id = NEXT_TRACE_ID.load(NEXT_TRACE_ID_ORDERING);
        NEXT_TRACE_ID.store(id + 1, NEXT_TRACE_ID_ORDERING);
        Arc::new(Self {
            id,
            parent,
            indent,
            kind,
        })
    }

    fn mock(
        id: usize,
        parent: Option<usize>,
        indent: Indent,
        tokens: Vec<TokenProps>,
    ) -> Arc<Self> {
        NEXT_TRACE_ID.store(id, NEXT_TRACE_ID_ORDERING);
        Arc::new(Self {
            id,
            parent,
            indent,
            kind: TraceKind::Mock { tokens },
        })
    }

    pub fn main(main_file: FilePtr, feature_block: Arc<FeatureBlock>) -> Arc<Self> {
        Self::new(
            None,
            0,
            TraceKind::Main {
                main_file,
                feature_block,
            },
        )
    }

    pub fn tokens(&self) -> Cow<[TokenProps]> {
        match self.kind {
            TraceKind::Mock { ref tokens } => tokens.into(),
            TraceKind::Main { .. } => Cow::Borrowed(&[TokenProps {
                kind: TraceTokenKind::Keyword,
                value: Cow::Borrowed("main"),
                spaces_before: Some(0),
            }]),
            TraceKind::Stmt(ref stmt) => eval_feature_stmt_trace_tokens(stmt).into(),
            TraceKind::Expr(ref expr) => {
                let mut tokens = eval_feature_expr_trace_tokens(expr);
                tokens[0].spaces_before = Some(0);
                tokens.into()
            }
        }
    }
}
