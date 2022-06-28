use semantics_eager::EagerOpnVariant;

use crate::*;

impl<'eval> Trace {
    pub fn subtraces_container_class(&self) -> Option<SubtracesContainerClass> {
        if !self.raw_data.can_have_subtraces {
            return None;
        }
        match self.variant {
            TraceVariant::Main(_)
            | TraceVariant::FeatureLazyStmt(_)
            | TraceVariant::FeatureLazyBranch(_)
            | TraceVariant::FeatureCallArgument { .. }
            | TraceVariant::FuncStmt { .. }
            | TraceVariant::ProcStmt { .. }
            | TraceVariant::LoopFrame { .. }
            | TraceVariant::CallHead { .. }
            | TraceVariant::ProcBranch { .. } => None,
            TraceVariant::FeatureLazyExpr(ref expr) => match expr.variant {
                FeatureLazyExprVariant::RoutineCall { .. } => Some(SubtracesContainerClass::Call),
                FeatureLazyExprVariant::EntityFeature { .. } => None,
                FeatureLazyExprVariant::RecordDerivedFieldAccess { .. }
                | FeatureLazyExprVariant::StructDerivedLazyFieldAccess { .. } => {
                    Some(SubtracesContainerClass::Call)
                }
                _ => None,
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
                        EagerOpnVariant::TypeCall { .. } => todo!(),
                    },
                    EagerExprVariant::Lambda(_, _) => todo!(),
                    EagerExprVariant::Bracketed(_) => panic!(),
                    EagerExprVariant::ThisValue { .. } => todo!(),
                    EagerExprVariant::ThisField { .. } => todo!(),
                    EagerExprVariant::EnumKindLiteral(_) => todo!(),
                }
            }
            TraceVariant::EagerCallArgument {
                name: ident,
                ref argument,
                ref history,
            } => todo!(),
        }
    }
}

#[derive(Serialize, Clone, Copy, PartialEq, Eq)]
pub enum SubtracesContainerClass {
    Call,
}
