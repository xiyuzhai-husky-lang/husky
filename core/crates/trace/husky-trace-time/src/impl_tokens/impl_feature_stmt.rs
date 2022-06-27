use super::*;

impl HuskyTraceTime {
    pub(crate) fn feature_stmt_tokens(&mut self, stmt: &FeatureStmt) -> Vec<TraceTokenData> {
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
}
