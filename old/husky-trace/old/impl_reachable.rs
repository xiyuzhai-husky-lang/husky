use crate::*;

impl TraceVariant {
    pub fn reachable(&self) -> bool {
        match self {
            TraceVariant::Main(..)
            | TraceVariant::EntityVal { .. }
            | TraceVariant::Module { .. } => true,
            TraceVariant::ValStmt(_) | TraceVariant::ValBranch(_) | TraceVariant::LazyExpr(_) => {
                true
            }
            TraceVariant::ValCallArgument { .. } | TraceVariant::EagerCallArgument { .. } => true,
            TraceVariant::EagerStmt {
                stmt,
                history,
                eager_expr_region,
            } => {
                todo!()
                //     match stmt.variant {
                //     HirEagerStmt::Init {
                //         ref initial_value, ..
                //     } => history.contains(initial_value),
                //     HirEagerStmt::Assert { ref condition } => history.contains(condition),
                //     HirEagerStmt::Execute { ref expr } => history.contains(expr),
                //     HirEagerStmt::ConditionFlow { .. } => panic!("ProcBranch"),
                //     HirEagerStmt::Loop { .. } | HirEagerStmt::Break => history.contains(stmt),
                //     HirEagerStmt::Return { ref result, .. } => history.contains(result),
                //     HirEagerStmt::Match { .. } => todo!(),
                // }
            }
            TraceVariant::LoopFrame { .. } => true,
            TraceVariant::EagerExpr { expr, history } => todo!(),
            // history.contains(expr),
            TraceVariant::CallHead { .. } => true,
            TraceVariant::EagerBranch {
                stmt,
                branch_idx,
                history,
                ..
            } => {
                todo!()
                // if let Some(entry) = history.get(stmt) {
                //     match entry {
                //         HistoryEntry::ControlFlow {
                //             opt_branch_entered, ..
                //         } => {
                //             if let Some(branch_entered) = opt_branch_entered {
                //                 if branch_idx > branch_entered {
                //                     false
                //                 } else {
                //                     true
                //                 }
                //             } else {
                //                 *branch_idx == 0
                //             }
                //         }
                //         _ => panic!(),
                //     }
                // } else {
                //     false
                // }
            }
        }
    }
}
