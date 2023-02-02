mod eager_expr;
mod feature_expr;
mod feature_repr;
mod feature_stmt;
mod func_stmt;
mod proc_stmt;
mod utils;

use crate::*;
use husky_comptime::*;
use husky_doc::TextQueryGroup;
use husky_eager_semantics::{EagerExpr, FuncStmt, FuncStmtVariant, ProcStmt, ProcStmtVariant};
use husky_feature_eval::EvalFeature;
use husky_feature_gen::{FeatureLazyExpr, FeatureLazyStmt, FeatureLazyStmtVariant};
use husky_vm::{History, HistoryEntry, MutationData, MutationDataVariant};

impl Debugtime {
    pub fn gen_trace_generic_figure(
        &self,
        trace_id: TraceId,
    ) -> Result<GenericFigureCanvasData, (SampleId, __VMError)> {
        let trace = self.trace(trace_id);
        Ok(match trace.variant {
            TraceVariant::Main(_) | TraceVariant::Module { .. } => Default::default(),
            TraceVariant::FeatureStmt(ref stmt) => self.feature_stmt_generic_figure(stmt)?,
            TraceVariant::FeatureBranch(_) => Default::default(),
            TraceVariant::EntityFeature { ref repr, .. } => {
                self.feature_repr_generic_figure(repr)?
            }
            TraceVariant::FeatureExpr(ref expr) => self.feature_expr_generic_figure(expr)?,
            TraceVariant::FeatureCallArgument {
                argument: ref input,
                ..
            } => self.feature_expr_generic_figure(input)?,
            TraceVariant::FuncStmt {
                stmt: _,
                history: _,
            } => todo!(),
            // self.func_stmt_generic_figure(stmt, history),
            TraceVariant::ProcStmt {
                stmt: _,
                history: _,
            } => todo!(),
            // self.proc_stmt_figure(stmt, history).into(),
            TraceVariant::EagerExpr {
                expr: _,
                history: _,
            } => todo!(),
            // self.eager_expr_figure(expr, history).into(),
            TraceVariant::CallHead { .. } => Default::default(),
            TraceVariant::LoopFrame { .. } => todo!(),
            // self
            //     .loop_frame_mutations_figure(
            //         trace.raw_data.opt_parent_id.unwrap(),
            //         &loop_frame_data.mutations,
            //     )
            //     .into(),
            TraceVariant::FuncBranch { .. } => todo!(),
            // match history.get(stmt) {
            //     Some(HistoryEntry::ControlFlow {
            //         opt_branch_entered: branch_entered,
            //         control,
            //         ..
            //     }) => {
            //         if *branch_entered == Some(branch_idx) {
            //             self.visualize_control(control)
            //         } else {
            //             FigureCanvasData::void()
            //         }
            //     }
            //     None => Default::default(),
            //     _ => panic!(),
            // },
            TraceVariant::ProcBranch { .. } => todo!(),
            // match history.get(stmt) {
            //     Some(HistoryEntry::ControlFlow {
            //         opt_branch_entered: branch_entered,
            //         mutations,
            //         ..
            //     }) => {
            //         if *branch_entered == Some(branch_idx) {
            //             self.mutations_figure(mutations).into()
            //         } else {
            //             FigureCanvasData::void()
            //         }
            //     }
            //     None => Default::default(),
            //     _ => panic!(),
            // },
            TraceVariant::EagerCallArgument { .. } => todo!(),
            //  self.eager_expr_figure(argument, history).into(),
        })
    }

    pub fn gen_trace_specific_figure(
        &self,
        trace_id: TraceId,
    ) -> Result<SpecificFigureCanvasData, (SampleId, __VMError)> {
        let trace = self.trace(trace_id);
        Ok(match trace.variant {
            TraceVariant::Main(_) | TraceVariant::Module { .. } => Default::default(),
            TraceVariant::FeatureStmt(ref stmt) => self.feature_stmt_specific_figure(stmt)?,
            TraceVariant::FeatureBranch(_) => Default::default(),
            TraceVariant::EntityFeature { ref repr, .. } => {
                self.feature_repr_specific_figure(repr)?
            }
            TraceVariant::FeatureExpr(ref expr) => self.feature_expr_specific_figure(expr)?,
            TraceVariant::FeatureCallArgument { ref argument, .. } => {
                self.feature_expr_specific_figure(argument)?
            }
            TraceVariant::FuncStmt {
                ref stmt,
                ref history,
            } => self.func_stmt_figure(stmt, history),
            TraceVariant::ProcStmt {
                ref stmt,
                ref history,
            } => self.proc_stmt_figure(stmt, history).into(),
            TraceVariant::EagerExpr {
                ref expr,
                ref history,
            } => self.eager_expr_figure(expr, history).into(),
            TraceVariant::CallHead { .. } => Default::default(),
            TraceVariant::LoopFrame {
                ref loop_frame_data,
                ..
            } => self
                .loop_frame_mutations_figure(
                    trace.raw_data.opt_parent_id.unwrap(),
                    &loop_frame_data.mutations,
                )
                .into(),
            TraceVariant::FuncBranch {
                ref stmt,
                branch_idx,
                ref history,
                ..
            } => match history.get(stmt) {
                Some(HistoryEntry::ControlFlow {
                    opt_branch_entered: branch_entered,
                    control,
                    ..
                }) => {
                    if *branch_entered == Some(branch_idx) {
                        self.visualize_control(control)
                    } else {
                        Default::default()
                    }
                }
                None => Default::default(),
                _ => panic!(),
            },
            TraceVariant::ProcBranch {
                ref stmt,
                branch_idx,
                ref history,
                ..
            } => match history.get(stmt) {
                Some(HistoryEntry::ControlFlow {
                    opt_branch_entered: branch_entered,
                    mutations,
                    ..
                }) => {
                    if *branch_entered == Some(branch_idx) {
                        self.mutations_figure(mutations).into()
                    } else {
                        Default::default()
                    }
                }
                None => Default::default(),
                _ => panic!(),
            },
            TraceVariant::EagerCallArgument {
                ref argument,
                ref history,
                ..
            } => self.eager_expr_figure(argument, history).into(),
        })
    }
}
