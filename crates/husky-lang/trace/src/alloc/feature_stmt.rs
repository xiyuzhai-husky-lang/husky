use feature::{FeatureStmt, FeatureStmtKind};

use crate::*;

use super::*;

impl TraceAllocator {
    pub fn feature_stmt_trace(&self, parent: Option<usize>, stmt: Arc<FeatureStmt>) -> Arc<Trace> {
        self.new_trace(parent, stmt.indent, TraceKind::Stmt(stmt))
    }

    pub fn feature_stmt_subtraces(
        &self,
        parent: usize,
        stmt: &FeatureStmt,
    ) -> Arc<Vec<Arc<Trace>>> {
        Arc::new(match stmt.kind {
            FeatureStmtKind::Init { varname, ref value } => {
                vec![self.feature_expr_trace(parent, stmt.indent, value.clone())]
            }
            FeatureStmtKind::Assert { ref condition } => {
                vec![self.feature_expr_trace(parent, stmt.indent, condition.clone())]
            }
            FeatureStmtKind::Return { ref result } => {
                vec![self.feature_expr_trace(parent, stmt.indent, result.clone())]
            }
            FeatureStmtKind::Branches {
                ref kind,
                ref branches,
            } => branches
                .iter()
                .map(|branch| {
                    self.new_trace(
                        Some(parent),
                        stmt.indent + 2,
                        TraceKind::Branch(branch.clone()),
                    )
                })
                .collect(),
        })
    }
}
