mod impl_eager_expr;
mod impl_feature_expr;
mod impl_feature_stmt;
mod impl_func_stmt;
mod impl_proc_stmt;
mod utils;

use crate::*;
use husky_compile_time::*;
use husky_eager_semantics::{
    EagerExpr, EagerExprVariant, FuncStmt, FuncStmtVariant, ProcStmt, ProcStmtVariant,
};
use husky_feature_eval::EvalFeature;
use husky_feature_gen::{FeatureExpr, FeatureExprVariant, FeatureLazyStmtVariant, FeatureStmt};
use husky_print_utils::epin;
use husky_text::TextQueryGroup;
use map_collect::MapCollect;
use vm::{History, HistoryEntry, MutationData, MutationDataVariant, StackSnapshot};

impl HuskyTraceTime {
    pub fn figure_canvas(
        &self,
        trace_id: TraceId,
    ) -> Result<FigureCanvasData, (SampleId, __VMError)> {
        let trace = self.trace(trace_id);
        Ok(match trace.variant {
            TraceVariant::Main(_) => FigureCanvasData::void(),
            TraceVariant::FeatureLazyStmt(ref stmt) => self.feature_stmt_figure(stmt)?,
            TraceVariant::FeatureLazyBranch(_) => FigureCanvasData::void(),
            TraceVariant::FeatureLazyExpr(ref expr) => self.feature_expr_figure(expr)?,
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
