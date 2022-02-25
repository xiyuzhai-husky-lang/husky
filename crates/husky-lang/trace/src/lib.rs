mod alloc;
mod figure;
mod interpreter;
mod kind;
mod stalk;
mod subtraces;
#[cfg(test)]
mod tests;
mod token;

pub use alloc::{AllocateTrace, TraceAllocator, TraceId};
pub use figure::FigureProps;
use file::FilePtr;
pub use interpreter::TraceInterpreter;
use interpreter::TraceInterpreterControlSignal;
pub use kind::TraceKind;
pub use stalk::TraceStalk;
use text::{Text, TextRange};
pub use token::{TokenProps, TraceTokenKind};

use std::{borrow::Cow, sync::Arc};

use common::*;
use fold::Indent;
use serde::{ser::SerializeStruct, Serialize};

// ts: { idx: number; parent: number | null; tokens: Token[] }
#[derive(Debug, Clone)]
pub struct Trace {
    parent: Option<TraceId>,
    pub(crate) id: TraceId,
    pub indent: fold::Indent,
    pub kind: TraceKind,
    pub tokens: Vec<TokenProps>,
    pub range: TextRange,
    pub file: FilePtr,
}

impl PartialEq for Trace {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Trace {}

impl Serialize for Trace {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Trace", 3)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("parent", &self.parent)?;
        state.serialize_field("indent", &self.indent)?;
        state.serialize_field("tokens", &self.tokens)?;
        state.serialize_field("kind", &self.kind)?;
        state.serialize_field(
            "has_subtraces",
            &match self.kind {
                TraceKind::FeatureStmt(_) | TraceKind::Input(_) | TraceKind::DeclStmt { .. } => {
                    false
                }
                TraceKind::Main(_) | TraceKind::FeatureBranch(_) => true,
                TraceKind::FeatureExpr(ref expr) => match expr.kind {
                    feature::FeatureExprKind::Literal(_)
                    | feature::FeatureExprKind::PrimitiveBinaryOpr { .. }
                    | feature::FeatureExprKind::Variable { .. } => false,
                    feature::FeatureExprKind::FuncCall { func, .. } => !func.is_builtin(),
                },
            },
        )?;
        state.serialize_field(
            "subtraces_container_class",
            &self.subtraces_container_class(),
        )?;
        state.end()
    }
}

impl Trace {
    pub(crate) fn new(
        parent: Option<TraceId>,
        indent: Indent,
        kind: TraceKind,
        trace_allocator: &TraceAllocator,
        text: &Text,
    ) -> Self {
        let id = trace_allocator.next_id();
        let (file, range) = match kind {
            TraceKind::Main(ref block) => (block.file, block.range),
            TraceKind::FeatureStmt(ref stmt) => (stmt.file, stmt.range),
            TraceKind::FeatureExpr(ref expr) => (expr.file, expr.range),
            TraceKind::FeatureBranch(ref branch) => (branch.block.file, branch.block.range),
            TraceKind::Input(_) => todo!(),
            TraceKind::DeclStmt { ref stmt, .. } => (stmt.file, stmt.range),
        };
        Self {
            id,
            parent,
            indent,
            tokens: trace_allocator.tokens(id, indent, &kind, text),
            kind,
            file,
            range,
        }
    }

    pub(crate) fn new2(
        parent: Option<TraceId>,
        indent: Indent,
        gen_kind: impl FnOnce(TraceId) -> TraceKind,
        trace_allocator: &TraceAllocator,
        text: &Text,
    ) -> Self {
        let id = trace_allocator.next_id();
        let kind = gen_kind(id);
        let (file, range) = match kind {
            TraceKind::Main(ref block) => (block.file, block.range),
            TraceKind::FeatureStmt(ref stmt) => (stmt.file, stmt.range),
            TraceKind::FeatureExpr(ref expr) => (expr.file, expr.range),
            TraceKind::FeatureBranch(ref branch) => (branch.block.file, branch.block.range),
            TraceKind::Input(_) => todo!(),
            TraceKind::DeclStmt { ref stmt, .. } => (stmt.file, stmt.range),
        };
        Self {
            id,
            parent,
            indent,
            tokens: trace_allocator.tokens(id, indent, &kind, text),
            kind,
            file,
            range,
        }
    }

    pub fn id(&self) -> TraceId {
        self.id
    }
}
