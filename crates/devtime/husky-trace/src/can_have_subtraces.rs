use crate::*;

impl TraceVariant {
    pub fn can_have_subtraces(&self, reachable: bool) -> bool {
        todo!()
        // if !reachable {
        //     return false;
        // }
        // match self {
        //     TraceVariant::FeatureStmt(_)
        //     | TraceVariant::FeatureCallArgument { .. }
        //     | TraceVariant::FuncStmt { .. }
        //     | TraceVariant::EagerCallArgument { .. } => false,
        //     TraceVariant::ProcStmt { ref stmt, .. } => match stmt.variant {
        //         HirEagerStmt::Init { .. }
        //         | HirEagerStmt::Assert { .. }
        //         | HirEagerStmt::Execute { .. }
        //         | HirEagerStmt::Return { .. } => false,
        //         HirEagerStmt::Loop { .. } => true,
        //         HirEagerStmt::ConditionFlow { .. } => panic!(),
        //         HirEagerStmt::Break => false,
        //         HirEagerStmt::Match { .. } => todo!(),
        //     },
        //     TraceVariant::LoopFrame { .. }
        //     | TraceVariant::Main(..)
        //     | TraceVariant::Module { .. }
        //     | TraceVariant::FeatureBranch(_) => true,
        //     TraceVariant::EntityFeature { repr, .. } => match repr {
        //         ValRepr::Value { .. } => false,
        //         ValRepr::LazyExpr(_) => false,
        //         ValRepr::LazyBody(_) => true,
        //         ValRepr::FuncBody(_) => true,
        //         ValRepr::ProcBody(_) => true,
        //         ValRepr::TargetInput { .. } => false,
        //     },
        //     TraceVariant::FeatureExpr(expr) => match expr.variant {
        //         FeatureLazyExprVariant::Literal(_)
        //         | FeatureLazyExprVariant::PrimitiveBinaryOpr { .. }
        //         | FeatureLazyExprVariant::ShortCircuitBinaryOpr { .. }
        //         | FeatureLazyExprVariant::Variable { .. } => false,
        //         FeatureLazyExprVariant::StructOriginalField { .. } => false,
        //         FeatureLazyExprVariant::EntityFeature { .. } => true,
        //         FeatureLazyExprVariant::NewRecord { .. } => todo!(),
        //         FeatureLazyExprVariant::RecordOriginalField { .. } => false,
        //         FeatureLazyExprVariant::ThisValue { .. } => false,
        //         FeatureLazyExprVariant::EvalInput => false,
        //         FeatureLazyExprVariant::RoutineCall {
        //             ref routine_defn, ..
        //         } => !routine_defn.is_builtin(),
        //         FeatureLazyExprVariant::RecordDerivedField { .. } => todo!(),
        //         FeatureLazyExprVariant::Index { .. } => false,
        //         FeatureLazyExprVariant::StructDerivedLazyField { .. } => true,
        //         FeatureLazyExprVariant::ModelCall { ref model_defn, .. } => {
        //             match model_defn.variant {
        //                 EntityDefnVariant::Function { ref source, .. } => match source {
        //                     CallFormSource::Lazy { .. } => true,
        //                     CallFormSource::Static(_) => false,
        //                     _ => panic!(),
        //                 },
        //                 _ => todo!(),
        //             }
        //         }
        //         FeatureLazyExprVariant::NewVecFromList { .. } => false,
        //         FeatureLazyExprVariant::CustomBinaryOpr {
        //             ref opt_instruction_sheet,
        //             ..
        //         } => opt_instruction_sheet.is_some(),
        //         FeatureLazyExprVariant::BePattern { .. } => false,
        //     },
        //     TraceVariant::EagerExpr { ref expr, .. } => match expr.variant {
        //         EagerExprVariant::Variable { .. } | EagerExprVariant::PrimitiveLiteral(_) => false,
        //         EagerExprVariant::Bracketed(_) => todo!(),
        //         EagerExprVariant::Opn {
        //             ref opn_variant,
        //             ref opds,
        //             ..
        //         } => match opn_variant {
        //             EagerOpnVariant::RoutineCall(ranged_route) => !ranged_route.route.is_builtin(),
        //             EagerOpnVariant::TypeCall { .. } => todo!(),
        //             // !ranged_ty.route.is_builtin(),
        //             EagerOpnVariant::Field { .. } => false,
        //             EagerOpnVariant::Binary { .. }
        //             | EagerOpnVariant::Prefix { .. }
        //             | EagerOpnVariant::Suffix { .. }
        //             | EagerOpnVariant::MethodCall { .. } => !opds[0].intrinsic_ty().is_builtin(),
        //             EagerOpnVariant::Index { .. } => false,
        //             EagerOpnVariant::NewVecFromList => todo!(),
        //             EagerOpnVariant::ValueCall => todo!(),
        //         },
        //         EagerExprVariant::Lambda(_, _)
        //         | EagerExprVariant::ThisValue { .. }
        //         | EagerExprVariant::ThisField { .. }
        //         | EagerExprVariant::EnumKindLiteral(_) => false,
        //         EagerExprVariant::EntityFeature { .. } => true,
        //         EagerExprVariant::EntityThickFp { .. } => todo!(),
        //     },
        //     TraceVariant::CallHead { .. } => false,
        //     TraceVariant::ProcBranch {
        //         stmt,
        //         branch_idx,
        //         history,
        //         ..
        //     } => {
        //         if let Some(entry) = history.get(stmt) {
        //             match entry {
        //                 HistoryEntry::ControlFlow {
        //                     opt_branch_entered: branch_entered,
        //                     ..
        //                 } => {
        //                     if *branch_entered == Some(*branch_idx) {
        //                         true
        //                     } else {
        //                         false
        //                     }
        //                 }
        //                 _ => panic!(),
        //             }
        //         } else {
        //             false
        //         }
        //     }
        //     TraceVariant::FuncBranch {
        //         stmt,
        //         branch_idx,
        //         history,
        //         ..
        //     } => {
        //         if let Some(entry) = history.get(stmt) {
        //             match entry {
        //                 HistoryEntry::ControlFlow {
        //                     opt_branch_entered: branch_entered,
        //                     ..
        //                 } => {
        //                     if *branch_entered == Some(*branch_idx) {
        //                         true
        //                     } else {
        //                         false
        //                     }
        //                 }
        //                 _ => panic!(),
        //             }
        //         } else {
        //             false
        //         }
        //     }
        // }
    }
}
