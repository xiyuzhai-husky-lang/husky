mod associated_traces;
mod factory;
mod figure;
mod kind;
mod stalk;
mod subtraces;
#[cfg(test)]
mod tests;
mod token;

pub use factory::{CreateTrace, TraceFactory, TraceId};
use feature::*;
pub use figure::FigureProps;
use file::FilePtr;
pub use kind::TraceVariant;
use print_utils::p;
use semantics_eager::*;
use semantics_entity::*;
pub use stalk::TraceStalk;
use text::{Text, TextRange};
pub use token::{TokenProps, TraceTokenKind};

use fold::Indent;
use serde::{ser::SerializeStruct, Serialize};
use std::{borrow::Cow, sync::Arc};

// ts: { idx: number; parent: number | null; tokens: Token[] }
#[derive(Debug, Clone)]
pub struct Trace<'eval> {
    parent: Option<TraceId>,
    pub(crate) id: TraceId,
    pub variant: TraceVariant<'eval>,
    pub indent: Indent,
    pub lines: Vec<LineProps<'eval>>,
    pub range: TextRange,
    pub file: FilePtr,
    pub compile_time_version: usize,
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
        state.serialize_field("kind", &self.variant)?;
        state.serialize_field("has_subtraces", &self.has_subtraces())?;
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
        kind: TraceVariant<'eval>,
        trace_allocator: &TraceFactory<'eval>,
        text: &Text,
        compile_time_version: usize,
    ) -> Self {
        let id = trace_allocator.next_id();
        let (file, range) = kind.file_and_range();
        Self {
            id,
            parent,
            indent,
            lines: trace_allocator.lines(id, indent, &kind, text),
            variant: kind,
            file,
            range,
            compile_time_version,
        }
    }

    pub fn id(&self) -> TraceId {
        self.id
    }

    pub fn has_subtraces(&self) -> bool {
        match self.variant {
            TraceVariant::FeatureStmt(_)
            | TraceVariant::FeatureCallInput { .. }
            | TraceVariant::FuncStmt { .. } => false,
            TraceVariant::ProcStmt { ref stmt, .. } => match stmt.variant {
                ProcStmtVariant::Init { .. }
                | ProcStmtVariant::Assert { .. }
                | ProcStmtVariant::Execute { .. }
                | ProcStmtVariant::Return { .. } => false,
                ProcStmtVariant::Loop { .. } => true,
                ProcStmtVariant::BranchGroup { .. } => panic!(),
            },
            TraceVariant::LoopFrame { .. }
            | TraceVariant::Main(_)
            | TraceVariant::FeatureBranch(_) => true,
            TraceVariant::FeatureExpr(ref expr) => match expr.kind {
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
                FeatureExprKind::ElementAccess { ref opds, .. } => false,
            },
            TraceVariant::EagerExpr { ref expr, .. } => match expr.variant {
                EagerExprVariant::Variable(_)
                | EagerExprVariant::Scope { .. }
                | EagerExprVariant::PrimitiveLiteral(_) => false,
                EagerExprVariant::Bracketed(_) => todo!(),
                EagerExprVariant::Opn {
                    ref opn_variant,
                    ref opds,
                    ..
                } => match opn_variant {
                    EagerOpnVariant::RoutineCall(_) => todo!(),
                    EagerOpnVariant::TypeCall { ranged_ty, .. } => !ranged_ty.route.is_builtin(),
                    EagerOpnVariant::PatternCall => todo!(),
                    EagerOpnVariant::FieldAccess { .. } => false,
                    EagerOpnVariant::Binary { .. }
                    | EagerOpnVariant::Prefix { .. }
                    | EagerOpnVariant::Suffix { .. }
                    | EagerOpnVariant::MethodCall { .. }
                    | EagerOpnVariant::ElementAccess => !opds[0].ty.is_builtin(),
                },
                EagerExprVariant::Lambda(_, _) => todo!(),
                EagerExprVariant::This => todo!(),
            },
            TraceVariant::CallHead { .. } => false,
        }
    }
}
