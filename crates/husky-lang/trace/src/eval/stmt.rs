use feature::FeatureStmt;

use crate::*;

pub fn eval_stmt_trace(parent: Option<usize>, stmt: Arc<FeatureStmt>) -> Arc<Trace> {
    Trace::new(parent, TraceKind::Stmt(stmt))
}
