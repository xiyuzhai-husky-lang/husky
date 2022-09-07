use super::*;

impl<'a> TraceLineBuilder<'a> {
    pub(crate) fn feature_branch_tokens(&mut self, branch: &FeatureLazyBranch) {
        match branch.variant {
            FeatureLazyBranchVariant::If { ref condition } => {
                self.gen_keyword_token("if ", None);
                self.gen_feature_expr_tokens(condition, ExprTokenConfig::branch())
            }
            FeatureLazyBranchVariant::Elif { ref condition } => {
                self.gen_keyword_token("elif ", None);
                self.gen_feature_expr_tokens(condition, ExprTokenConfig::branch())
            }
            FeatureLazyBranchVariant::Else => self.gen_keyword_token("else ", None),
        }
    }
}
