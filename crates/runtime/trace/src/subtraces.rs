use semantics_eager::EagerOpnKind;

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
                    ref opn_kind,
                    ref opds,
                    ..
                } => match opn_kind {
                    EagerOpnKind::FieldAccess { .. } | EagerOpnKind::ElementAccess => None,
                    EagerOpnKind::Binary { .. }
                    | EagerOpnKind::Prefix { .. }
                    | EagerOpnKind::Suffix { .. } => {
                        if opds[0].ty.is_builtin() {
                            None
                        } else {
                            Some(SubtracesContainerClass::Call)
                        }
                    }
                    EagerOpnKind::RoutineCall { .. } | EagerOpnKind::MethodCall { .. } => {
                        Some(SubtracesContainerClass::Call)
                    }
                    EagerOpnKind::PatternCall => panic!(),
                    EagerOpnKind::TypeCall { .. } => todo!(),
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
