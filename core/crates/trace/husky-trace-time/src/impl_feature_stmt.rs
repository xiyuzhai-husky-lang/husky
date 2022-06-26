use crate::*;

use super::{impl_tokens::ExprTokenConfig, *};

impl HuskyTraceTime {
    pub fn feature_lazy_stmt_traces(
        &mut self,
        parent: &Trace,
        stmt: Arc<FeatureLazyStmt>,
    ) -> Vec<TraceId> {
        match stmt.variant {
            FeatureLazyStmtVariant::Init { .. }
            | FeatureLazyStmtVariant::Assert { .. }
            | FeatureLazyStmtVariant::Return { .. } => {
                vec![self.new_trace(
                    Some(parent.id()),
                    stmt.indent,
                    TraceVariant::FeatureLazyStmt(stmt),
                )]
            }
            FeatureLazyStmtVariant::ConditionFlow { ref branches, .. } => branches
                .iter()
                .map(|branch| self.feature_branch_trace(parent, stmt.indent, branch.clone()))
                .collect(),
            FeatureLazyStmtVariant::ReturnXml { ref result } => todo!(),
        }
    }

    pub fn feature_stmt_lines(&mut self, stmt: &FeatureLazyStmt) -> Vec<TraceLineData> {
        vec![TraceLineData {
            indent: stmt.indent,
            idx: 0,
            tokens: self.feature_stmt_tokens(stmt),
        }]
    }

    pub fn feature_stmt_tokens(&mut self, stmt: &FeatureLazyStmt) -> Vec<TraceTokenData> {
        match stmt.variant {
            FeatureLazyStmtVariant::Init { varname, ref value } => {
                let mut tokens = vec![];
                tokens.push(ident!(varname.0));
                tokens.push(special!(" = "));
                tokens.extend(self.feature_expr_tokens(value, ExprTokenConfig::stmt()));
                tokens
            }
            FeatureLazyStmtVariant::Assert { ref condition } => {
                let mut tokens = vec![];
                tokens.push(keyword!("assert "));
                tokens.extend(self.feature_expr_tokens(condition, ExprTokenConfig::stmt()));
                tokens
            }
            FeatureLazyStmtVariant::Return { ref result } => {
                let mut tokens = vec![];
                tokens.extend(self.feature_expr_tokens(result, ExprTokenConfig::stmt()));
                tokens
            }
            FeatureLazyStmtVariant::ConditionFlow { .. } => panic!(),
            FeatureLazyStmtVariant::ReturnXml { ref result } => todo!(),
        }
    }
    pub(crate) fn feature_stmt_figure(
        &self,
        stmt: &FeatureLazyStmt,
        attention: &Attention,
    ) -> Result<FigureCanvasData, (SampleId, EvalError)> {
        match stmt.variant {
            FeatureLazyStmtVariant::Init { varname, ref value } => {
                self.feature_expr_figure(value, attention)
            }
            FeatureLazyStmtVariant::Assert { .. } => Ok(FigureCanvasData::void()),
            FeatureLazyStmtVariant::Return { ref result } => {
                self.feature_expr_figure(result, attention)
            }
            FeatureLazyStmtVariant::ConditionFlow { ref branches } => todo!(),
            FeatureLazyStmtVariant::ReturnXml { ref result } => todo!(),
        }
    }
}
