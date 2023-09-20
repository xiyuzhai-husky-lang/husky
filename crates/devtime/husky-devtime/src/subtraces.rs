mod expr_subtraces;
mod impl_feature_branch;
mod impl_feature_repr;
mod impl_func_stmt;
mod impl_module;
mod impl_proc_stmt;

use super::*;
use husky_check_utils::should_eq;

impl<Task: IsTask> Devtime<Task> {
    pub fn gen_subtraces(&mut self, trace_id: TraceId) -> Option<Vec<TraceId>> {
        todo!()
        // let trace = unsafe { self.trace_ref(trace_id) };
        // match trace.variant {
        //     TraceVariant::Main(ref repr) => self.feature_repr_subtraces(&trace, repr),
        //     TraceVariant::Module { route, .. } => Some(self.module_subtraces(&trace, route)),
        //     TraceVariant::EntityFeature { ref repr, .. } => {
        //         self.feature_repr_subtraces(&trace, repr)
        //     }
        //     TraceVariant::FeatureStmt(_)
        //     | TraceVariant::FeatureCallArgument { .. }
        //     | TraceVariant::FuncStmt { .. }
        //     | TraceVariant::CallHead { .. } => None,
        //     TraceVariant::ProcStmt {
        //         ref stmt,
        //         ref history,
        //     } => match stmt.variant {
        //         HirEagerStmt::Init { .. }
        //         | HirEagerStmt::Assert { .. }
        //         | HirEagerStmt::Execute { .. }
        //         | HirEagerStmt::Return { .. } => None,
        //         HirEagerStmt::ConditionFlow { .. } => panic!(),
        //         HirEagerStmt::Loop { ref stmts, .. } => {
        //             match history
        //                 .get(stmt)
        //                 .expect("if there is no entry, there is no subtraces")
        //             {
        //                 HistoryEntry::PureExpr { .. } | HistoryEntry::Exec { .. } => None,
        //                 HistoryEntry::Loop {
        //                     ref stack_snapshot,
        //                     body_instruction_sheet: ref body,
        //                     loop_kind,
        //                     ..
        //                 } => Some(self.loop_subtraces(
        //                     trace,
        //                     *loop_kind,
        //                     stmt,
        //                     stmts,
        //                     stack_snapshot,
        //                     body,
        //                 )),
        //                 HistoryEntry::ControlFlow { .. } => todo!(),
        //                 HistoryEntry::Break => todo!(),
        //                 HistoryEntry::PatternMatching { .. } => todo!(),
        //             }
        //         }
        //         HirEagerStmt::Break => None,
        //         HirEagerStmt::Match { .. } => todo!(),
        //     },
        //     TraceVariant::FeatureExpr(ref expr) => self.feature_expr_subtraces(trace, expr),
        //     TraceVariant::FeatureBranch(ref branch) => {
        //         Some(self.feature_lazy_block_subtraces(trace, &branch.block))
        //     }
        //     TraceVariant::EagerExpr {
        //         ref expr,
        //         ref history,
        //     } => Some(self.eager_expr_subtraces(trace, expr, history)),
        //     TraceVariant::LoopFrame {
        //         ref loop_frame_data,
        //         ref loop_stmt,
        //         ref body_stmts,
        //         ref body_instruction_sheet,
        //     } => Some(self.loop_frame_subtraces(
        //         loop_stmt,
        //         body_stmts,
        //         body_instruction_sheet,
        //         loop_frame_data,
        //         trace,
        //     )),
        //     TraceVariant::FuncBranch {
        //         ref stmt,
        //         branch_idx,
        //         ref history,
        //         ref opt_vm_branch,
        //         ref branch,
        //         ..
        //     } => match history.get(stmt).unwrap() {
        //         HistoryEntry::ControlFlow {
        //             stack_snapshot,
        //             opt_branch_entered: branch_entered,
        //             ..
        //         } => {
        //             should_eq!(Some(branch_idx), *branch_entered);
        //             Some(self.func_branch_subtraces(
        //                 &branch.stmts,
        //                 &opt_vm_branch.as_ref().unwrap().body,
        //                 stack_snapshot,
        //                 trace,
        //             ))
        //         }
        //         _ => panic!(),
        //     },
        //     TraceVariant::ProcBranch {
        //         ref stmt,
        //         branch_idx,
        //         ref history,
        //         ref opt_vm_branch,
        //         ref branch,
        //         ..
        //     } => match history.get(stmt).unwrap() {
        //         HistoryEntry::ControlFlow {
        //             stack_snapshot,
        //             opt_branch_entered: branch_entered,
        //             ..
        //         } => {
        //             should_eq!(Some(branch_idx), *branch_entered);
        //             Some(self.proc_branch_subtraces(
        //                 &branch.stmts,
        //                 &opt_vm_branch.as_ref().unwrap().body,
        //                 stack_snapshot,
        //                 trace,
        //             ))
        //         }
        //         _ => panic!(),
        //     },
        //     TraceVariant::EagerCallArgument { .. } => todo!(),
        // }
    }
}
