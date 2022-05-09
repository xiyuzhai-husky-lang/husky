use crate::*;

use super::{expr::ExprTokenConfig, *};

impl<'eval> TraceFactory<'eval> {
    pub fn feature_stmt_traces(
        &self,
        parent: &Trace,
        stmt: Arc<FeatureStmt>,
        text: &Text,
    ) -> Vec<Arc<Trace<'eval>>> {
        match stmt.variant {
            FeatureStmtVariant::Init { .. }
            | FeatureStmtVariant::Assert { .. }
            | FeatureStmtVariant::Return { .. } => {
                vec![self.new_trace(
                    Some(parent.id),
                    stmt.indent,
                    TraceVariant::FeatureStmt(stmt),
                    text,
                )]
            }
            FeatureStmtVariant::BranchGroup { ref branches, .. } => branches
                .iter()
                .map(|branch| self.feature_branch_trace(parent, stmt.indent, branch.clone(), text))
                .collect(),
        }
    }

    pub fn feature_stmt_lines(&self, stmt: &FeatureStmt, text: &Text) -> Vec<LineProps<'eval>> {
        vec![LineProps {
            indent: stmt.indent,
            idx: 0,
            tokens: self.feature_stmt_tokens(stmt, text),
        }]
    }

    pub fn feature_stmt_tokens(&self, stmt: &FeatureStmt, text: &Text) -> Vec<TokenProps<'eval>> {
        match stmt.variant {
            FeatureStmtVariant::Init { varname, ref value } => {
                let mut tokens = vec![];
                tokens.push(ident!(varname.0));
                tokens.push(special!(" = "));
                tokens.extend(self.feature_expr_tokens(value, text, ExprTokenConfig::stmt()));
                tokens
            }
            FeatureStmtVariant::Assert { ref condition } => {
                let mut tokens = vec![];
                tokens.push(keyword!("assert "));
                tokens.extend(self.feature_expr_tokens(condition, text, ExprTokenConfig::stmt()));
                tokens
            }
            FeatureStmtVariant::Return { ref result } => {
                let mut tokens = vec![];
                tokens.extend(self.feature_expr_tokens(result, text, ExprTokenConfig::stmt()));
                tokens
            }
            FeatureStmtVariant::BranchGroup { .. } => panic!(),
        }
    }
}
