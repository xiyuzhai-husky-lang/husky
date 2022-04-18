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
use feature::*;
pub use kind::TraceKind;
use semantics_eager::*;
use semantics_entity::*;
pub use stalk::TraceStalk;
use text::{Text, TextRange};
pub use token::{TokenProps, TraceTokenKind};

use fold::Indent;
use print_utils::*;
use serde::{ser::SerializeStruct, Serialize};
use std::{borrow::Cow, sync::Arc};

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
                    ProcStmtKind::Init { .. }
                    | ProcStmtKind::Assert { .. }
                    | ProcStmtKind::Execute { .. }
                    | ProcStmtKind::Return { .. } => false,
                    ProcStmtKind::Loop { .. } => true,
                    ProcStmtKind::BranchGroup { .. } => panic!(),
                },
                TraceKind::LoopFrame { .. } | TraceKind::Main(_) | TraceKind::FeatureBranch(_) => {
                    true
                }
                TraceKind::FeatureExpr(ref expr) => match expr.kind {
                    FeatureExprKind::PrimitiveLiteral(_)
                    | FeatureExprKind::PrimitiveBinaryOpr { .. }
                    | FeatureExprKind::Variable { .. } => false,
                    FeatureExprKind::StructOriginalFieldAccess { .. } => todo!(),
                    FeatureExprKind::EnumLiteral { .. } => todo!(),
                    FeatureExprKind::EntityFeature { .. } => todo!(),
                    FeatureExprKind::NewRecord { ty, ref opds, .. } => todo!(),
                    FeatureExprKind::RecordOriginalFieldAccess {
                        ref this,
                        field_ident,
                        ..
                    } => todo!(),
                    FeatureExprKind::This { ref repr } => todo!(),
                    FeatureExprKind::GlobalInput => false,
                    FeatureExprKind::RoutineCall {
                        ref routine_defn, ..
                    } => !routine_defn.is_builtin(),
                    FeatureExprKind::PatternCall {} => true,
                    FeatureExprKind::RecordDerivedFieldAccess { .. } => todo!(),
                },
                TraceKind::EagerExpr { ref expr, .. } => match expr.kind {
                    EagerExprKind::Variable(_)
                    | EagerExprKind::Scope { .. }
                    | EagerExprKind::PrimitiveLiteral(_) => false,
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

    pub fn id(&self) -> TraceId {
        self.id
    }

    pub fn compile_time_version(&self) -> usize {
        todo!()
    }
}
