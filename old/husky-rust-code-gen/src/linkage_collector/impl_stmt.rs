use super::*;

impl<'a> LinkageCollector<'a> {
    // pub(crate) fn collect_from_lazy_stmts(&mut self, stmts: &[HirLazyStmtIdx]) {
    //     for stmt in stmts {
    //         match stmt.variant {
    //             LazyStmtVariant::Init { ref value, .. } => self.collect_from_lazy_expr(value),
    //             LazyStmtVariant::Assert { ref condition } => self.collect_from_lazy_expr(condition),
    //             LazyStmtVariant::Require { ref condition, .. } => {
    //                 self.collect_from_lazy_expr(condition)
    //             }
    //             LazyStmtVariant::Return { ref result } => self.collect_from_lazy_expr(result),
    //             LazyStmtVariant::ReturnUnveil { ref result, .. } => {
    //                 self.collect_from_lazy_expr(result)
    //             }
    //             LazyStmtVariant::ReturnHtml { .. } => (),
    //             LazyStmtVariant::ConditionFlow { ref branches, .. } => {
    //                 for branch in branches {
    //                     match branch.variant {
    //                         LazyConditionBranchVariant::If { ref condition } => {
    //                             self.collect_from_lazy_expr(condition)
    //                         }
    //                         LazyConditionBranchVariant::Elif { ref condition } => {
    //                             self.collect_from_lazy_expr(condition)
    //                         }
    //                         LazyConditionBranchVariant::Else => (),
    //                     }
    //                     self.collect_from_lazy_stmts(&branch.stmts)
    //                 }
    //             }
    //             LazyStmtVariant::Match { .. } => todo!(),
    //         }
    //     }
    // }

    // pub(crate) fn collect_from_func_stmts(&mut self, stmts: &[HirEagerStmtIdx]) {
    //     for stmt in stmts {
    //         match stmt.variant {
    //             FuncStmtVariant::Init {
    //                 ref initial_value, ..
    //             } => self.collect_from_eager_expr(initial_value),
    //             FuncStmtVariant::Assert { ref condition } => {
    //                 self.collect_from_eager_expr(condition)
    //             }
    //             FuncStmtVariant::Require { ref condition, .. } => {
    //                 self.collect_from_eager_expr(condition)
    //             }
    //             FuncStmtVariant::Return { ref result, .. } => self.collect_from_eager_expr(result),
    //             FuncStmtVariant::ConditionFlow { ref branches } => {
    //                 for branch in branches {
    //                     match branch.variant {
    //                         FuncConditionFlowBranchVariant::If { ref condition } => {
    //                             self.collect_from_eager_expr(condition)
    //                         }
    //                         FuncConditionFlowBranchVariant::Elif { ref condition } => {
    //                             self.collect_from_eager_expr(condition)
    //                         }
    //                         FuncConditionFlowBranchVariant::Else => (),
    //                     }
    //                     self.collect_from_func_stmts(&branch.stmts)
    //                 }
    //             }
    //             FuncStmtVariant::Match {
    //                 ref match_expr,
    //                 ref branches,
    //             } => {
    //                 self.collect_from_eager_expr(match_expr);
    //                 for branch in branches {
    //                     self.collect_from_func_stmts(&branch.stmts)
    //                 }
    //             }
    //         }
    //     }
    // }

    // pub(crate) fn collect_from_proc_stmts(&mut self, stmts: &[HirEagerStmtIdx]) {
    //     for stmt in stmts {
    //         match stmt.variant {
    //             HirEagerStmt::Init {
    //                 ref initial_value, ..
    //             } => self.collect_from_eager_expr(initial_value),
    //             HirEagerStmt::Assert { ref condition } => {
    //                 self.collect_from_eager_expr(condition)
    //             }
    //             HirEagerStmt::Execute { ref expr } => self.collect_from_eager_expr(expr),
    //             HirEagerStmt::ConditionFlow { ref branches } => {
    //                 for branch in branches {
    //                     match branch.variant {
    //                         ProcConditionFlowBranchVariant::If { ref condition } => {
    //                             self.collect_from_eager_expr(condition)
    //                         }
    //                         ProcConditionFlowBranchVariant::Elif { ref condition } => {
    //                             self.collect_from_eager_expr(condition)
    //                         }
    //                         ProcConditionFlowBranchVariant::Else => (),
    //                     }
    //                     self.collect_from_proc_stmts(&branch.stmts)
    //                 }
    //             }
    //             HirEagerStmt::Loop {
    //                 ref loop_variant,
    //                 ref stmts,
    //             } => {
    //                 match loop_variant {
    //                     LoopVariant::For {
    //                         initial_boundary,
    //                         final_boundary,
    //                         ..
    //                     } => {
    //                         self.collect_from_boundary(initial_boundary);
    //                         self.collect_from_boundary(final_boundary)
    //                     }
    //                     LoopVariant::ForExt { final_boundary, .. } => {
    //                         self.collect_from_boundary(final_boundary)
    //                     }
    //                     LoopVariant::While { condition } => self.collect_from_eager_expr(condition),
    //                     LoopVariant::DoWhile { condition } => {
    //                         self.collect_from_eager_expr(condition)
    //                     }
    //                 }
    //                 self.collect_from_proc_stmts(stmts)
    //             }
    //             HirEagerStmt::Break => (),
    //             HirEagerStmt::Return { ref result, .. } => self.collect_from_eager_expr(result),
    //             HirEagerStmt::Match {
    //                 ref match_expr,
    //                 ref branches,
    //             } => {
    //                 self.collect_from_eager_expr(match_expr);
    //                 for branch in branches {
    //                     self.collect_from_proc_stmts(&branch.stmts)
    //                 }
    //             }
    //         }
    //     }
    // }

    // fn collect_from_boundary(&mut self, boundary: &Boundary) {
    //     if let Some(ref bound) = boundary.opt_bound {
    //         self.collect_from_eager_expr(bound)
    //     }
    // }
}
