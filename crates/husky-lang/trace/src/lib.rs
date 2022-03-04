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
use interpreter::TraceInterpreterControlSignal;
pub use interpreter::{TraceInterpreter, TraceStackValue};
pub use kind::TraceKind;
use semantics::ImprStmtKind;
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
                TraceKind::FeatureStmt(_)
                | TraceKind::Input(_)
                | TraceKind::StrictDeclStmt { .. } => false,
                TraceKind::ImprStmt { ref stmt, .. } => match stmt.kind {
                    ImprStmtKind::Init { .. }
                    | ImprStmtKind::Assert { .. }
                    | ImprStmtKind::Return { .. } => false,
                    ImprStmtKind::Loop => true,
                    ImprStmtKind::BranchGroup { .. } => panic!(),
                },
                TraceKind::Main(_) | TraceKind::FeatureBranch(_) => true,
                TraceKind::FeatureExpr(ref expr) => match expr.kind {
                    feature::FeatureExprKind::Literal(_)
                    | feature::FeatureExprKind::PrimitiveBinaryOpr { .. }
                    | feature::FeatureExprKind::Variable { .. } => false,
                    feature::FeatureExprKind::FuncCall { ranged_scope, .. } => {
                        !ranged_scope.scope.is_builtin()
                    }
                    feature::FeatureExprKind::ProcCall { ranged_scope, .. } => {
                        !ranged_scope.scope.is_builtin()
                    }
                },
                TraceKind::Expr { ref expr, .. } => match expr.kind {
                    semantics::ExprKind::Variable(_)
                    | semantics::ExprKind::Scope { .. }
                    | semantics::ExprKind::Literal(_) => false,
                    semantics::ExprKind::Bracketed(_) => todo!(),
                    semantics::ExprKind::Opn { ref opds, .. } => !opds[0].ty.is_builtin(),
                    semantics::ExprKind::Lambda(_, _) => todo!(),
                },
                TraceKind::CallHead { .. } => false,
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
        let (file, range) = kind.file_and_range();
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
        let (file, range) = kind.file_and_range();
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
