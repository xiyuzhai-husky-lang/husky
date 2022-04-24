use crate::*;
use compile_time_db::*;
use feature::{FeatureExpr, FeatureExprKind, FeatureStmt, FeatureStmtKind};

impl HuskyLangRuntime {
    pub fn figure(&self, trace_id: TraceId, focus: &Focus) -> FigureProps {
        let trace = self.trace(trace_id);
        match trace.kind {
            TraceKind::Main(_) => FigureProps::void(),
            TraceKind::FeatureStmt(ref stmt) => self.feature_stmt_figure(stmt, focus),
            TraceKind::FeatureBranch(_) => FigureProps::void(),
            TraceKind::FeatureExpr(ref expr) => self.feature_expr_figure(expr, focus),
            TraceKind::Input(_) => todo!(),
            TraceKind::StrictDeclStmt {
                ref stmt,
                ref history,
            } => todo!(),
            TraceKind::ImprStmt {
                ref stmt,
                ref history,
            } => todo!(),
            TraceKind::LoopFrame { .. } => todo!(),
            TraceKind::EagerExpr {
                ref expr,
                ref history,
            } => todo!(),
            TraceKind::CallHead {
                ref entity,
                ref tokens,
            } => todo!(),
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
}
