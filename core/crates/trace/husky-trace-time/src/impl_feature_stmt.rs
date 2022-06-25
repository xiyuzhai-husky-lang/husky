use crate::*;

use super::{impl_expr::ExprTokenConfig, *};

impl HuskyTraceTime {
    pub fn feature_stmt_traces(&mut self, parent: &Trace, stmt: Arc<FeatureStmt>) -> Vec<TraceId> {
        match stmt.variant {
            FeatureStmtVariant::Init { .. }
            | FeatureStmtVariant::Assert { .. }
            | FeatureStmtVariant::Return { .. } => {
                vec![self.new_trace(
                    Some(parent.id()),
                    stmt.indent,
                    TraceVariant::FeatureStmt(stmt),
                )]
            }
            FeatureStmtVariant::ConditionFlow { ref branches, .. } => branches
                .iter()
                .map(|branch| self.feature_branch_trace(parent, stmt.indent, branch.clone()))
                .collect(),
            FeatureStmtVariant::ReturnXml { ref result } => todo!(),
        }
    }

    pub fn feature_stmt_lines(&mut self, stmt: &FeatureStmt) -> Vec<TraceLineData> {
        vec![TraceLineData {
            indent: stmt.indent,
            idx: 0,
            tokens: self.feature_stmt_tokens(stmt),
        }]
    }

    pub fn feature_stmt_tokens(&mut self, stmt: &FeatureStmt) -> Vec<TraceTokenData> {
        match stmt.variant {
            FeatureStmtVariant::Init { varname, ref value } => {
                let mut tokens = vec![];
                tokens.push(ident!(varname.0));
                tokens.push(special!(" = "));
                tokens.extend(self.feature_expr_tokens(value, ExprTokenConfig::stmt()));
                tokens
            }
            FeatureStmtVariant::Assert { ref condition } => {
                let mut tokens = vec![];
                tokens.push(keyword!("assert "));
                tokens.extend(self.feature_expr_tokens(condition, ExprTokenConfig::stmt()));
                tokens
            }
            FeatureStmtVariant::Return { ref result } => {
                let mut tokens = vec![];
                tokens.extend(self.feature_expr_tokens(result, ExprTokenConfig::stmt()));
                tokens
            }
            FeatureStmtVariant::ConditionFlow { .. } => panic!(),
            FeatureStmtVariant::ReturnXml { ref result } => todo!(),
        }
    }
    pub(crate) fn feature_stmt_figure(
        &self,
        stmt: &FeatureStmt,
        attention: &Attention,
    ) -> Result<FigureCanvasData, (SampleId, EvalError)> {
        match stmt.variant {
            FeatureStmtVariant::Init { varname, ref value } => {
                self.feature_expr_figure(value, attention)
            }
            FeatureStmtVariant::Assert { .. } => Ok(FigureCanvasData::void()),
            FeatureStmtVariant::Return { ref result } => {
                self.feature_expr_figure(result, attention)
            }
            FeatureStmtVariant::ConditionFlow { ref branches } => todo!(),
            FeatureStmtVariant::ReturnXml { ref result } => todo!(),
        }
    }
}
