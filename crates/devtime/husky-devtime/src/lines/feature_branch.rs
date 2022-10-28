use super::*;

impl<'a> TraceLineGenerator<'a> {
    pub(crate) fn gen_feature_branch(&mut self, branch: &FeatureLazyBranch) {
        match branch.variant {
            FeatureLazyBranchVariant::If { ref condition } => {
                self.render_keyword_token("if ", None, None);
                self.gen_feature_expr(condition, ExprTokenConfig::branch())
            }
            FeatureLazyBranchVariant::Elif { ref condition } => {
                self.render_keyword_token("elif ", None, None);
                self.gen_feature_expr(condition, ExprTokenConfig::branch())
            }
            FeatureLazyBranchVariant::Else => self.render_keyword_token("else ", None, None),
        }
    }
}
