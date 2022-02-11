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

    pub(crate) fn feature_stmt_tokens(
        &self,
        stmt_trace_id: TraceId,
        stmt: &FeatureStmt,
    ) -> Vec<TokenProps> {
        match stmt.kind {
            FeatureStmtKind::Init { varname, ref value } => {
                let mut tokens = vec![];
                tokens.push(ident!(varname.0, None));
                tokens.push(special!(" = ", None));
                tokens.extend(self.feature_expr_tokens(value, true));
                tokens.into()
            }
            FeatureStmtKind::Assert { ref condition } => {
                let mut tokens = vec![];
                tokens.push(keyword!("assert "));
                tokens.extend(self.feature_expr_tokens(condition, true));
                tokens.into()
            }
            FeatureStmtKind::Return { ref result } => {
                let mut tokens = vec![];
                tokens.extend(self.feature_expr_tokens(result, true));
                tokens.into()
            }
            FeatureStmtKind::Branches { .. } => panic!(),
        }
    }
}
