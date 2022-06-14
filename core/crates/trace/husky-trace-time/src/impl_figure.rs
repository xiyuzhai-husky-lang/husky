use crate::*;
use eval_feature::EvalFeature;
use feature_gen::{FeatureExpr, FeatureExprVariant, FeatureStmt, FeatureStmtVariant};
use husky_compile_time::*;
use map_collect::MapCollect;
use semantics_eager::{
    EagerExpr, EagerExprVariant, FuncStmt, FuncStmtVariant, ProcStmt, ProcStmtVariant,
};
use text::TextQueryGroup;
use vm::{History, HistoryEntry, MutationData, MutationDataKind, StackSnapshot};

impl HuskyTraceTime {
    pub fn figure_canvas_data(&self, trace_id: TraceId, focus: &Focus) -> FigureCanvasData {
        let trace = self.trace(trace_id);
        match trace.variant {
            TraceVariant::Main(_) => FigureCanvasData::void(),
            TraceVariant::FeatureStmt(ref stmt) => self.feature_stmt_figure(stmt, focus),
            TraceVariant::FeatureBranch(_) => FigureCanvasData::void(),
            TraceVariant::FeatureExpr(ref expr) => self.feature_expr_figure(expr, focus),
            TraceVariant::FeatureCallInput { ref input, .. } => {
                self.feature_expr_figure(input, focus)
            }
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
                trace.props.opt_parent_id.unwrap(),
                &loop_frame_data.mutations,
                &loop_frame_data.stack_snapshot,
            ),
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
        }
    }
}
