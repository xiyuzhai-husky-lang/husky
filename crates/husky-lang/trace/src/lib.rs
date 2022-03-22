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
pub struct Trace<'eval> {
    parent: Option<TraceId>,
    pub(crate) id: TraceId,
    pub kind: TraceKind<'eval>,
    pub indent: Indent,
    pub lines: Vec<LineProps<'eval>>,
    pub range: TextRange,
    pub file: FilePtr,
}

#[derive(Debug, Clone, Serialize)]
pub struct LineProps<'eval> {
    pub indent: Indent,
    pub tokens: Vec<TokenProps<'eval>>,
    pub idx: usize,
}

impl<'eval> PartialEq for Trace<'eval> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<'eval> Eq for Trace<'eval> {}

impl<'eval> Serialize for Trace<'eval> {
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
                    FeatureExprKind::PrimitiveLiteral(_)
                    | FeatureExprKind::PrimitiveBinaryOpr { .. }
                    | FeatureExprKind::Variable { .. } => false,
                    FeatureExprKind::FuncCall { ranged_scope, .. } => {
                        !ranged_scope.scope.is_builtin()
                    }
                    FeatureExprKind::ProcCall { ranged_scope, .. } => {
                        !ranged_scope.scope.is_builtin()
                    }
                    FeatureExprKind::MembVarAccess { .. } => todo!(),
                    FeatureExprKind::EnumLiteral { .. } => todo!(),
                },
                TraceKind::EagerExpr { ref expr, .. } => match expr.kind {
                    EagerExprKind::Variable(_)
                    | EagerExprKind::Scope { .. }
                    | EagerExprKind::Literal(_) => false,
                    EagerExprKind::Bracketed(_) => todo!(),
                    EagerExprKind::Opn { ref opds, .. } => !opds[0].ty.is_builtin(),
                    EagerExprKind::Lambda(_, _) => todo!(),
                    EagerExprKind::This => todo!(),
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

impl<'eval> Trace<'eval> {
    pub(crate) fn new(
        parent: Option<TraceId>,
        indent: Indent,
        kind: TraceKind<'eval>,
        trace_allocator: &TraceFactory<'eval>,
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
