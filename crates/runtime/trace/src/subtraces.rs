use semantics_eager::EagerOpnVariant;

use crate::*;

impl<'eval> Trace<'eval> {
    pub fn subtraces_container_class(&self) -> Option<SubtracesContainerClass> {
        if !self.has_subtraces {
            return None;
        }
        match self.variant {
            TraceVariant::Main(_)
            | TraceVariant::FeatureStmt(_)
            | TraceVariant::FeatureBranch(_)
            | TraceVariant::FeatureCallInput { .. }
            | TraceVariant::FuncStmt { .. }
            | TraceVariant::ProcStmt { .. }
            | TraceVariant::LoopFrame { .. }
            | TraceVariant::CallHead { .. }
            | TraceVariant::ProcBranch { .. } => None,
            TraceVariant::FeatureExpr(ref expr) => match expr.variant {
                FeatureExprVariant::PrimitiveLiteral(_)
                | FeatureExprVariant::PrimitiveBinaryOpr { .. }
                | FeatureExprVariant::Variable { .. }
                | FeatureExprVariant::ElementAccess { .. }
                | FeatureExprVariant::GlobalInput => None,
                FeatureExprVariant::RoutineCall { .. } => Some(SubtracesContainerClass::Call),
                FeatureExprVariant::StructOriginalFieldAccess { .. } => todo!(),
                FeatureExprVariant::EnumKindLiteral { .. } => todo!(),
                FeatureExprVariant::EntityFeature { .. } => todo!(),
                FeatureExprVariant::NewRecord { ty, ref opds, .. } => todo!(),
                FeatureExprVariant::RecordOriginalFieldAccess {
                    ref this,
                    field_ident,
                    ..
                } => todo!(),
                FeatureExprVariant::This { ref repr } => todo!(),
                FeatureExprVariant::PatternCall {} => todo!(),
                FeatureExprVariant::RecordDerivedFieldAccess { .. } => todo!(),
                FeatureExprVariant::StructDerivedFieldAccess {
                    ref this,
                    field_ident,
                    ref block,
                } => todo!(),
            },
            TraceVariant::EagerExpr { ref expr, .. } => {
                match expr.variant {
                    EagerExprVariant::Variable { .. }
                    | EagerExprVariant::EntityRoute { .. }
                    | EagerExprVariant::PrimitiveLiteral(_) => None,
                    EagerExprVariant::Opn {
                        opn_variant: ref opn_kind,
                        ref opds,
                        ..
                    } => match opn_kind {
                        EagerOpnVariant::FieldAccess { .. }
                        | EagerOpnVariant::ElementAccess { .. } => None,
                        EagerOpnVariant::Binary { .. }
                        | EagerOpnVariant::Prefix { .. }
                        | EagerOpnVariant::Suffix { .. } => {
                            if opds[0].ty().is_builtin() {
                                None
                            } else {
                                Some(SubtracesContainerClass::Call)
                            }
                        }
                        EagerOpnVariant::RoutineCall { .. }
                        | EagerOpnVariant::MethodCall { .. } => Some(SubtracesContainerClass::Call),
                        EagerOpnVariant::PatternCall => panic!(),
                        EagerOpnVariant::TypeCall { .. } => todo!(),
                    },
                    EagerExprVariant::Lambda(_, _) => todo!(),
                    EagerExprVariant::Bracketed(_) => panic!(),
                    EagerExprVariant::ThisValue { .. } => todo!(),
                    EagerExprVariant::ThisField { .. } => todo!(),
                    EagerExprVariant::EnumKindLiteral(_) => todo!(),
                }
            }
        }
    }
}

#[derive(Serialize, Clone, Copy, PartialEq, Eq)]
pub enum SubtracesContainerClass {
    Call,
}
