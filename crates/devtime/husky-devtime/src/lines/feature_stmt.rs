use super::*;

impl<'a> TraceLineBuilder<'a> {
    pub(crate) fn feature_stmt_tokens(&mut self, stmt: &FeatureLazyStmt) {
        match stmt.variant {
            FeatureLazyStmtVariant::Init { varname, ref value } => {
                self.gen_ident_token(varname.0, None, None);
                self.gen_special_token(" = ", None, None);
                self.gen_feature_expr_tokens(value, ExprTokenConfig::stmt())
            }
            FeatureLazyStmtVariant::Assert { ref condition } => {
                self.gen_keyword_token("assert ", None, None);
                self.gen_feature_expr_tokens(condition, ExprTokenConfig::stmt())
            }
            FeatureLazyStmtVariant::Require { ref condition, .. } => {
                self.gen_keyword_token("require ", None, None);
                self.gen_feature_expr_tokens(condition, ExprTokenConfig::stmt())
            }
            FeatureLazyStmtVariant::Return { ref result } => {
                self.gen_feature_expr_tokens(result, ExprTokenConfig::stmt())
            }
            FeatureLazyStmtVariant::ReturnUnveil { ref result, .. } => {
                self.gen_feature_expr_tokens(result, ExprTokenConfig::stmt());
                self.gen_special_token("?", None, None)
            }
            FeatureLazyStmtVariant::ConditionFlow { .. } => panic!(),
            FeatureLazyStmtVariant::ReturnXml { .. } => todo!(),
        }
    }
}
