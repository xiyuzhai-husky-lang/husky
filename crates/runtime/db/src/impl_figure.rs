use crate::*;
use compile_time_db::*;
use feature::{FeatureExpr, FeatureExprKind, FeatureStmt, FeatureStmtKind};
use semantics_eager::{
    EagerExpr, EagerExprVariant, FuncStmt, FuncStmtVariant, ProcStmt, ProcStmtVariant,
};
use vm::{History, HistoryEntry};

impl HuskyLangRuntime {
    pub fn figure(&self, trace_id: TraceId, focus: &Focus) -> FigureProps {
        let trace = self.trace(trace_id);
        match trace.variant {
            TraceVariant::Main(_) => FigureProps::void(),
            TraceVariant::FeatureStmt(ref stmt) => self.feature_stmt_figure(stmt, focus),
            TraceVariant::FeatureBranch(_) => FigureProps::void(),
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
            } => FigureProps::void(),
            TraceVariant::LoopFrame { .. } => todo!(),
        }
    }

    fn feature_stmt_figure(&self, stmt: &FeatureStmt, focus: &Focus) -> FigureProps {
        match stmt.kind {
            FeatureStmtKind::Init { varname, ref value } => self.feature_expr_figure(value, focus),
            FeatureStmtKind::Assert { .. } => FigureProps::void(),
            FeatureStmtKind::Return { ref result } => self.feature_expr_figure(result, focus),
            FeatureStmtKind::BranchGroup { kind, ref branches } => FigureProps::void(),
        }
    }

    fn feature_expr_figure(&self, expr: &FeatureExpr, focus: &Focus) -> FigureProps {
        match focus.opt_input_id {
            Some(input_id) => {
                if let Ok(value) = self.eval_feature_expr(expr, input_id) {
                    let visualizer = self.visualizer(self.version(), expr.expr.ty);
                    let visual_props = visualizer.visualize(value.any_ref());
                    FigureProps::new_specific(visual_props)
                } else {
                    FigureProps::void()
                }
            }
            None => todo!(),
        }
    }

    fn func_stmt_figure(&self, stmt: &FuncStmt, history: &History) -> FigureProps {
        match stmt.variant {
            FuncStmtVariant::Init {
                varname,
                ref initial_value,
            } => todo!(),
            FuncStmtVariant::Assert { ref condition } => todo!(),
            FuncStmtVariant::Return { ref result } => todo!(),
            FuncStmtVariant::Branches { kind, ref branches } => todo!(),
        }
    }

    fn proc_stmt_figure(&self, stmt: &ProcStmt, history: &History) -> FigureProps {
        match stmt.variant {
            ProcStmtVariant::Init {
                varname,
                ref initial_value,
                init_kind,
                varidx,
            } => self.eager_expr_figure(initial_value, history),
            ProcStmtVariant::Assert { ref condition } => todo!(),
            ProcStmtVariant::Execute { ref expr } => todo!(),
            ProcStmtVariant::Return { ref result } => self.eager_expr_figure(result, history),
            ProcStmtVariant::BranchGroup { kind, ref branches } => todo!(),
            ProcStmtVariant::Loop {
                ref loop_variant,
                ref stmts,
            } => todo!(),
        }
    }

    fn eager_expr_figure(&self, expr: &EagerExpr, history: &History) -> FigureProps {
        let visualizer = self.visualizer(self.version(), expr.ty);
        match history.entry(expr) {
            HistoryEntry::NonVoidExpr { output } => {
                let visual_props = visualizer.visualize(output.any_ref());
                FigureProps::new_specific(visual_props)
            }
            HistoryEntry::Exec => todo!(),
            HistoryEntry::Assign { before, after } => todo!(),
            HistoryEntry::Loop {
                result,
                stack_snapshot,
                body,
            } => todo!(),
        }
    }
}
