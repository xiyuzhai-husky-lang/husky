use super::*;

impl<'a> TraceTokenBuilder<'a> {
    pub(crate) fn feature_stmt_tokens(&mut self, stmt: &FeatureLazyStmt) {
        match stmt.variant {
            FeatureLazyStmtVariant::Init { varname, ref value } => {
                self.push(ident!(varname.0));
                self.push(special!(" = "));
                self.gen_feature_expr_tokens(value, ExprTokenConfig::stmt())
            }
            FeatureLazyStmtVariant::Assert { ref condition } => {
                self.push(keyword!("assert "));
                self.gen_feature_expr_tokens(condition, ExprTokenConfig::stmt())
            }
            FeatureLazyStmtVariant::Require { ref condition, .. } => {
                self.push(keyword!("require "));
                self.gen_feature_expr_tokens(condition, ExprTokenConfig::stmt())
            }
            FeatureLazyStmtVariant::Return { ref result } => {
                self.gen_feature_expr_tokens(result, ExprTokenConfig::stmt())
            }
            FeatureLazyStmtVariant::ReturnUnveil { ref result, .. } => {
                self.gen_feature_expr_tokens(result, ExprTokenConfig::stmt());
                self.push(TraceTokenData {
                    kind: TraceTokenKind::Special,
                    value: "?".to_string(),
                    opt_associated_trace_id: None,
                })
            }
            FeatureLazyStmtVariant::ConditionFlow { .. } => panic!(),
            FeatureLazyStmtVariant::ReturnXml { .. } => todo!(),
        }
    }
}
