use semantics_eager::EagerOpnVariant;

use crate::*;

impl<'eval> Trace<'eval> {
    pub fn subtraces_container_class(&self) -> Option<SubtracesContainerClass> {
        match self.variant {
            TraceVariant::Main(_)
            | TraceVariant::FeatureStmt(_)
            | TraceVariant::FeatureBranch(_)
            | TraceVariant::FeatureCallInput { .. }
            | TraceVariant::FuncStmt { .. }
            | TraceVariant::ProcStmt { .. }
            | TraceVariant::LoopFrame { .. }
            | TraceVariant::CallHead { .. } => None,
            TraceVariant::FeatureExpr(ref expr) => match expr.kind {
                FeatureExprKind::PrimitiveLiteral(_)
                | FeatureExprKind::PrimitiveBinaryOpr { .. }
                | FeatureExprKind::Variable { .. }
                | FeatureExprKind::ElementAccess { .. }
                | FeatureExprKind::GlobalInput => None,
                FeatureExprKind::RoutineCall { .. } => Some(SubtracesContainerClass::Call),
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
                FeatureExprKind::PatternCall {} => todo!(),
                FeatureExprKind::RecordDerivedFieldAccess { .. } => todo!(),
            },
            TraceVariant::EagerExpr { ref expr, .. } => match expr.variant {
                EagerExprVariant::Variable(_)
                | EagerExprVariant::Scope { .. }
                | EagerExprVariant::PrimitiveLiteral(_) => None,
                EagerExprVariant::Opn {
                    opn_variant: ref opn_kind,
                    ref opds,
                    ..
                } => match opn_kind {
                    EagerOpnVariant::FieldAccess { .. } | EagerOpnVariant::ElementAccess => None,
                    EagerOpnVariant::Binary { .. }
                    | EagerOpnVariant::Prefix { .. }
                    | EagerOpnVariant::Suffix { .. } => {
                        if opds[0].ty.is_builtin() {
                            None
                        } else {
                            Some(SubtracesContainerClass::Call)
                        }
                    }
                    EagerOpnVariant::RoutineCall { .. } | EagerOpnVariant::MethodCall { .. } => {
                        Some(SubtracesContainerClass::Call)
                    }
                    EagerOpnVariant::PatternCall => panic!(),
                    EagerOpnVariant::TypeCall { .. } => todo!(),
                },
                EagerExprVariant::Lambda(_, _) => todo!(),
                EagerExprVariant::Bracketed(_) => panic!(),
                EagerExprVariant::This => todo!(),
            },
        }
    }
}

#[derive(Serialize, Clone, Copy, PartialEq, Eq)]
pub enum SubtracesContainerClass {
    Call,
}
