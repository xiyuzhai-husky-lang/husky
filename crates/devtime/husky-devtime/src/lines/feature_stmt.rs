use super::*;

impl<'a> TraceLineGenerator<'a> {
    pub(crate) fn feature_stmt_tokens(&mut self, stmt: &ValStmt) {
        match stmt.variant {
            FeatureLazyStmtVariant::Init { varname, ref value } => {
                self.render_ident_token(varname.as_str(), None, None);
                self.render_special_token(" = ", None, None);
                self.gen_feature_expr(value, ExprTokenConfig::stmt())
            }
            FeatureLazyStmtVariant::Assert { ref condition } => {
                self.render_keyword_token("assert ", None, None);
                self.gen_feature_expr(condition, ExprTokenConfig::stmt())
            }
            FeatureLazyStmtVariant::Require { ref condition, .. } => {
                self.render_keyword_token("require ", None, None);
                self.gen_feature_expr(condition, ExprTokenConfig::stmt())
            }
            FeatureLazyStmtVariant::Return { ref result } => {
                self.gen_feature_expr(result, ExprTokenConfig::stmt())
            }
            FeatureLazyStmtVariant::ReturnUnveil { ref result, .. } => {
                self.gen_feature_expr(result, ExprTokenConfig::stmt());
                self.render_special_token("?", None, None)
            }
            FeatureLazyStmtVariant::ConditionFlow { .. } => panic!(),
            FeatureLazyStmtVariant::ReturnHtml { .. } => todo!(),
        }
    }
}
