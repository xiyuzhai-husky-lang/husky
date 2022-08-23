mod impl_eager_expr;
mod impl_feature_expr;
mod impl_feature_repr;
mod impl_feature_stmt;
mod impl_func_stmt;
mod impl_proc_stmt;
mod utils;

use crate::*;
use husky_comptime::*;
use husky_eager_semantics::{
    EagerExpr, EagerExprVariant, FuncStmt, FuncStmtVariant, ProcStmt, ProcStmtVariant,
};
use husky_feature_eval::EvalFeature;
use husky_feature_gen::{
    FeatureLazyExpr, FeatureLazyExprVariant, FeatureLazyStmt, FeatureLazyStmtVariant,
};
use husky_print_utils::epin;
use husky_text::TextQueryGroup;
use husky_vm::{History, HistoryEntry, MutationData, MutationDataVariant, StackSnapshot};
use map_collect::MapCollect;

impl HuskyTraceTime {
    pub fn figure_canvas(
        &self,
        trace_id: TraceId,
    ) -> Result<FigureCanvasData, (SampleId, __VMError)> {
        let trace = self.trace(trace_id);
        Ok(match trace.variant {
            TraceVariant::Main(_) | TraceVariant::Module { .. } => FigureCanvasData::void(),
            TraceVariant::FeatureStmt(ref stmt) => self.feature_stmt_figure(stmt)?,
            TraceVariant::FeatureBranch(_) => FigureCanvasData::void(),
            TraceVariant::EntityFeature { ref repr, .. } => self.feature_repr_figure(repr)?,
            TraceVariant::FeatureExpr(ref expr) => self.feature_expr_figure(expr)?,
            TraceVariant::FeatureCallArgument {
                argument: ref input,
                ..
            } => self.feature_expr_figure(input)?,
            TraceVariant::FuncStmt {
                ref stmt,
                ref history,
            } => self.func_stmt_figure(stmt, history),
            TraceVariant::ProcStmt {
                ref stmt,
                ref history,
            } => self.proc_stmt_figure(stmt, history),
            TraceVariant::EagerExpr {
                ref expr,
                ref history,
            } => self.eager_expr_figure(expr, history),
            TraceVariant::CallHead {
                ref entity,
                ref tokens,
            } => FigureCanvasData::void(),
            TraceVariant::LoopFrame {
                ref loop_frame_data,
                ..
            } => self.loop_frame_mutations_figure(
                trace.raw_data.opt_parent_id.unwrap(),
                &loop_frame_data.mutations,
                &loop_frame_data.stack_snapshot,
            ),
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
                        FigureCanvasData::void()
                    }
                }
                None => FigureCanvasData::void(),
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
                        self.mutations_figure(mutations)
                    } else {
                        FigureCanvasData::void()
                    }
                }
                None => FigureCanvasData::void(),
                _ => panic!(),
            },
            TraceVariant::EagerCallArgument {
                name: ident,
                ref argument,
                ref history,
            } => self.eager_expr_figure(argument, history),
        })
    }
}
