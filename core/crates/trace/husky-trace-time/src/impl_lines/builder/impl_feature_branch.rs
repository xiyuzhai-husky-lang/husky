use super::*;

impl<'a> TraceTokenBuilder<'a> {
    pub(crate) fn feature_branch_tokens(&mut self, branch: &FeatureLazyBranch) {
        match branch.variant {
            FeatureBranchVariant::If { ref condition } => {
                self.push(keyword!("if "));
                self.gen_feature_expr_tokens(condition, ExprTokenConfig::branch())
            }
            FeatureBranchVariant::Elif { ref condition } => {
                self.push(keyword!("elif "));
                self.gen_feature_expr_tokens(condition, ExprTokenConfig::branch())
            }
            FeatureBranchVariant::Else => self.push(keyword!("else ")),
        }
    }
}
