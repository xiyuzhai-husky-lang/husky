mod factory;
mod figure;
mod kind;
mod stalk;
mod subtraces;
#[cfg(test)]
mod tests;
mod token;

pub use factory::{CreateTrace, TraceFactory, TraceId};
pub use figure::FigureProps;
use file::FilePtr;
// use interpreter::VMControl;
// pub use interpreter::{TraceInterpreter, VMValueSnapshot};
pub use kind::TraceKind;
use semantics_eager::*;
use semantics_entity::*;
use semantics_feature::*;
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
    pub kind: TraceKind,
    pub indent: Indent,
    pub lines: Vec<LineProps>,
    pub range: TextRange,
    pub file: FilePtr,
}

#[derive(Debug, Clone, Serialize)]
pub struct LineProps {
    pub indent: Indent,
    pub tokens: Vec<TokenProps>,
    pub idx: usize,
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
        state.serialize_field("lines", &self.lines)?;
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
                    | ImprStmtKind::Execute { .. }
                    | ImprStmtKind::Return { .. } => false,
                    ImprStmtKind::Loop { .. } => true,
                    ImprStmtKind::BranchGroup { .. } => panic!(),
                },
                TraceKind::LoopFrame { .. } | TraceKind::Main(_) | TraceKind::FeatureBranch(_) => {
                    true
                }
                TraceKind::FeatureExpr(ref expr) => match expr.kind {
                    FeatureExprKind::Literal(_)
                    | FeatureExprKind::PrimitiveBinaryOpr { .. }
                    | FeatureExprKind::Variable { .. } => false,
                    FeatureExprKind::FuncCall { ranged_scope, .. } => {
                        !ranged_scope.scope.is_builtin()
                    }
                    FeatureExprKind::ProcCall { ranged_scope, .. } => {
                        !ranged_scope.scope.is_builtin()
                    }
                },
                TraceKind::EagerExpr { ref expr, .. } => match expr.kind {
                    EagerExprKind::Variable(_)
                    | EagerExprKind::Scope { .. }
                    | EagerExprKind::Literal(_) => false,
                    EagerExprKind::Bracketed(_) => todo!(),
                    EagerExprKind::Opn { ref opds, .. } => !opds[0].ty.is_builtin(),
                    EagerExprKind::Lambda(_, _) => todo!(),
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
        trace_allocator: &TraceFactory,
        text: &Text,
    ) -> Self {
        let id = trace_allocator.next_id();
        let (file, range) = kind.file_and_range();
        Self {
            id,
            parent,
            indent,
            lines: trace_allocator.lines(id, indent, &kind, text),
            kind,
            file,
            range,
        }
    }

    // pub(crate) fn new2(
    //     parent: Option<TraceId>,
    //     indent: Indent,
    //     gen_kind: impl FnOnce(TraceId) -> TraceKind,
    //     trace_allocator: &TraceFactory,
    //     text: &Text,
    // ) -> Self {
    //     let id = trace_allocator.next_id();
    //     let kind = gen_kind(id);
    //     let (file, range) = kind.file_and_range();
    //     Self {
    //         id,
    //         parent,
    //         line: trace_allocator.tokens(id, indent, &kind, text),
    //         kind,
    //         file,
    //         range,
    //     }
    // }

    pub fn id(&self) -> TraceId {
        self.id
    }
}
