use feature::{LazyStmt, LazyStmtKind};

use crate::*;

use super::{expr::ExprTokenConfig, *};

impl TraceFactory {
    pub fn lazy_stmt_traces(
        &self,
        parent: &Trace,
        stmt: Arc<LazyStmt>,
        text: &Text,
    ) -> Vec<Arc<Trace>> {
        match stmt.kind {
            LazyStmtKind::Init { .. }
            | LazyStmtKind::Assert { .. }
            | LazyStmtKind::Return { .. } => {
                vec![self.new_trace(
                    Some(parent.id),
                    stmt.indent,
                    TraceKind::LazyStmt(stmt),
                    text,
                )]
            }
            LazyStmtKind::Branches { ref branches, .. } => branches
                .iter()
                .map(|branch| self.lazy_branch_trace(parent, stmt.indent, branch.clone(), text))
                .collect(),
        }
    }

    pub fn lazy_stmt_tokens(&self, stmt: &LazyStmt, text: &Text) -> Vec<TokenProps> {
        match stmt.kind {
            LazyStmtKind::Init { varname, ref value } => {
                let mut tokens = vec![];
                tokens.push(ident!(varname.0));
                tokens.push(special!(" = "));
                tokens.extend(self.lazy_expr_tokens(value, text, ExprTokenConfig::stmt()));
                tokens.into()
            }
            LazyStmtKind::Assert { ref condition } => {
                let mut tokens = vec![];
                tokens.push(keyword!("assert "));
                tokens.extend(self.lazy_expr_tokens(condition, text, ExprTokenConfig::stmt()));
                tokens.into()
            }
            LazyStmtKind::Return { ref result } => {
                let mut tokens = vec![];
                tokens.extend(self.lazy_expr_tokens(result, text, ExprTokenConfig::stmt()));
                tokens.into()
            }
            LazyStmtKind::Branches { .. } => panic!(),
        }
    }
}
