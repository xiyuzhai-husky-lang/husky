use crate::*;

impl<'eval> TraceVariant<'eval> {
    pub fn can_have_subtraces(&self, reachable: bool) -> bool {
        if !reachable {
            return false;
        }
        match self {
            TraceVariant::FeatureStmt(_)
            | TraceVariant::FeatureCallArgument { .. }
            | TraceVariant::FuncStmt { .. }
            | TraceVariant::EagerCallArgument { .. } => false,
            TraceVariant::ProcStmt { ref stmt, .. } => match stmt.variant {
                ProcStmtVariant::Init { .. }
                | ProcStmtVariant::Assert { .. }
                | ProcStmtVariant::Execute { .. }
                | ProcStmtVariant::Return { .. } => false,
                ProcStmtVariant::Loop { .. } => true,
                ProcStmtVariant::ConditionFlow { .. } => panic!(),
                ProcStmtVariant::Break => false,
                ProcStmtVariant::Match {
                    ref match_expr,
                    ref branches,
                } => todo!(),
            },
            TraceVariant::LoopFrame { .. }
            | TraceVariant::Main(_)
            | TraceVariant::Module { .. }
            | TraceVariant::EntityFeature { .. }
            | TraceVariant::FeatureBranch(_) => true,
            TraceVariant::FeatureExpr(expr) => match expr.variant {
                FeatureLazyExprVariant::Literal(_)
                | FeatureLazyExprVariant::PrimitiveBinaryOpr { .. }
                | FeatureLazyExprVariant::Variable { .. } => false,
                FeatureLazyExprVariant::StructOriginalField { .. } => false,
                FeatureLazyExprVariant::EntityFeature { .. } => true,
                FeatureLazyExprVariant::NewRecord { ty, ref opds, .. } => todo!(),
                FeatureLazyExprVariant::RecordOriginalField { .. } => false,
                FeatureLazyExprVariant::ThisValue { .. } => false,
                FeatureLazyExprVariant::EvalInput => false,
                FeatureLazyExprVariant::RoutineCall {
                    ref routine_defn, ..
                } => !routine_defn.is_builtin(),
                FeatureLazyExprVariant::RecordDerivedField { .. } => todo!(),
                FeatureLazyExprVariant::ElementAccess { ref opds, .. } => false,
                FeatureLazyExprVariant::StructDerivedLazyField {
                    ref this,
                    field_ident,
                    field_uid,
                    ref repr,
                } => true,
                FeatureLazyExprVariant::ModelCall {
                    ref opds,
                    has_this,
                    ref model_defn,
                    ref internal,
                    ..
                } => match model_defn.variant {
                    EntityDefnVariant::Function { ref source, .. } => match source {
                        CallFormSource::Lazy { stmts } => true,
                        CallFormSource::Static(_) => false,
                        _ => panic!(),
                    },
                    _ => todo!(),
                },
                FeatureLazyExprVariant::NewVecFromList { .. } => false,
                FeatureLazyExprVariant::CustomBinaryOpr {
                    ref opt_instruction_sheet,
                    ..
                } => opt_instruction_sheet.is_some(),
                FeatureLazyExprVariant::BePattern {
                    ref this,
                    patt: ref pure_pattern,
                } => todo!(),
            },
            TraceVariant::EagerExpr {
                ref expr,
                ref history,
            } => match expr.variant {
                EagerExprVariant::Variable { .. } | EagerExprVariant::PrimitiveLiteral(_) => false,
                EagerExprVariant::Bracketed(_) => todo!(),
                EagerExprVariant::Opn {
                    ref opn_variant,
                    ref opds,
                    ..
                } => match opn_variant {
                    EagerOpnVariant::RoutineCall(ranged_route) => !ranged_route.route.is_builtin(),
                    EagerOpnVariant::TypeCall { ranged_ty, .. } => !ranged_ty.route.is_builtin(),
                    EagerOpnVariant::Field { .. } => false,
                    EagerOpnVariant::Binary { .. }
                    | EagerOpnVariant::Prefix { .. }
                    | EagerOpnVariant::Suffix { .. }
                    | EagerOpnVariant::MethodCall { .. } => !opds[0].ty().is_builtin(),
                    EagerOpnVariant::Index { .. } => false,
                    EagerOpnVariant::NewVecFromList => todo!(),
                    EagerOpnVariant::ValueCall => todo!(),
                },
                EagerExprVariant::Lambda(_, _)
                | EagerExprVariant::ThisValue { .. }
                | EagerExprVariant::ThisField { .. }
                | EagerExprVariant::EnumKindLiteral(_) => false,
                EagerExprVariant::EntityFeature { .. } => true,
                EagerExprVariant::EntityThickFp { route } => todo!(),
            },
            TraceVariant::CallHead { .. } => false,
            TraceVariant::ProcBranch {
                stmt,
                branch_idx,
                history,
                ..
            } => {
                if let Some(entry) = history.get(stmt) {
                    match entry {
                        HistoryEntry::ControlFlow {
                            opt_branch_entered: branch_entered,
                            ..
                        } => {
                            if *branch_entered == Some(*branch_idx) {
                                true
                            } else {
                                false
                            }
                        }
                        _ => panic!(),
                    }
                } else {
                    false
                }
            }
            TraceVariant::FuncBranch {
                stmt,
                branch_idx,
                history,
                ..
            } => {
                if let Some(entry) = history.get(stmt) {
                    match entry {
                        HistoryEntry::ControlFlow {
                            opt_branch_entered: branch_entered,
                            ..
                        } => {
                            if *branch_entered == Some(*branch_idx) {
                                true
                            } else {
                                false
                            }
                        }
                        _ => panic!(),
                    }
                } else {
                    false
                }
            }
        }
    }
}
