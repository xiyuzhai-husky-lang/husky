use feature::{FeatureExpr, FeatureExprKind, FeatureStmt, FeatureStmtKind};

use crate::*;

use super::*;

impl TraceAllocator {
    pub fn feature_stmt_traces(&self, parent: &Trace, stmt: Arc<FeatureStmt>) -> Vec<Arc<Trace>> {
        match stmt.kind {
            FeatureStmtKind::Init { .. }
            | FeatureStmtKind::Assert { .. }
            | FeatureStmtKind::Return { .. } => {
                vec![self.new_trace(Some(parent), stmt.indent, TraceKind::FeatureStmt(stmt))]
            }
            FeatureStmtKind::Branches { ref branches, .. } => branches
                .iter()
                .map(|branch| self.feature_branch_trace(parent, stmt.indent, branch.clone()))
                .collect(),
        }
    }
}
