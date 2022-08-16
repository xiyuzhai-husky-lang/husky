use super::*;

impl<'a> TraceTokenBuilder<'a> {
    pub(crate) fn feature_stmt_tokens(&mut self, stmt: &FeatureStmt) {
        match stmt.variant {
            FeatureStmtVariant::Init { varname, ref value } => {
                self.push(ident!(varname.0));
                self.push(special!(" = "));
                self.gen_feature_expr_tokens(value, ExprTokenConfig::stmt())
            }
            FeatureStmtVariant::Assert { ref condition } => {
                self.push(keyword!("assert "));
                self.gen_feature_expr_tokens(condition, ExprTokenConfig::stmt())
            }
            FeatureStmtVariant::Require { ref condition, .. } => {
                self.push(keyword!("require "));
                self.gen_feature_expr_tokens(condition, ExprTokenConfig::stmt())
            }
            FeatureStmtVariant::Return { ref result } => {
                self.gen_feature_expr_tokens(result, ExprTokenConfig::stmt())
            }
            FeatureStmtVariant::ConditionFlow { .. } => panic!(),
            FeatureStmtVariant::ReturnXml { ref result } => todo!(),
        }
    }
}
