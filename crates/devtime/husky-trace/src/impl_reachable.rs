use crate::*;

impl TraceVariant {
    pub fn reachable(&self) -> bool {
        match self {
            TraceVariant::Main(_)
            | TraceVariant::EntityFeature { .. }
            | TraceVariant::Module { .. } => true,
            TraceVariant::FeatureStmt(_)
            | TraceVariant::LazyBranch(_)
            | TraceVariant::FeatureExpr(_) => true,
            TraceVariant::FeatureCallArgument { .. } | TraceVariant::EagerCallArgument { .. } => {
                true
            }
            TraceVariant::EagerStmt {
                stmt,
                history,
                eager_expr_region,
            } => {
                todo!()
                //     match stmt.variant {
                //     ProcStmtVariant::Init {
                //         ref initial_value, ..
                //     } => history.contains(initial_value),
                //     ProcStmtVariant::Assert { ref condition } => history.contains(condition),
                //     ProcStmtVariant::Execute { ref expr } => history.contains(expr),
                //     ProcStmtVariant::ConditionFlow { .. } => panic!("ProcBranch"),
                //     ProcStmtVariant::Loop { .. } | ProcStmtVariant::Break => history.contains(stmt),
                //     ProcStmtVariant::Return { ref result, .. } => history.contains(result),
                //     ProcStmtVariant::Match { .. } => todo!(),
                // }
            }
            TraceVariant::LoopFrame { .. } => true,
            TraceVariant::EagerExpr { expr, history } => history.contains(expr),
            TraceVariant::CallHead { .. } => true,
            TraceVariant::EagerBranch {
                stmt,
                branch_idx,
                history,
                ..
            } => {
                if let Some(entry) = history.get(stmt) {
                    match entry {
                        HistoryEntry::ControlFlow {
                            opt_branch_entered, ..
                        } => {
                            if let Some(branch_entered) = opt_branch_entered {
                                if branch_idx > branch_entered {
                                    false
                                } else {
                                    true
                                }
                            } else {
                                *branch_idx == 0
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
                            opt_branch_entered, ..
                        } => {
                            if let Some(branch_entered) = opt_branch_entered {
                                if branch_idx > branch_entered {
                                    false
                                } else {
                                    true
                                }
                            } else {
                                *branch_idx == 0
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
