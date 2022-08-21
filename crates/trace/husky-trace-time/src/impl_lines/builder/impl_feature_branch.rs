use super::*;

impl<'a> TraceTokenBuilder<'a> {
    pub(crate) fn feature_branch_tokens(&mut self, branch: &FeatureLazyBranch) {
        match branch.variant {
            FeatureLazyBranchVariant::If { ref condition } => {
                self.push(keyword!("if "));
                self.gen_feature_expr_tokens(condition, ExprTokenConfig::branch())
            }
            FeatureLazyBranchVariant::Elif { ref condition } => {
                self.push(keyword!("elif "));
                self.gen_feature_expr_tokens(condition, ExprTokenConfig::branch())
            }
            FeatureLazyBranchVariant::Else => self.push(keyword!("else ")),
        }
    }
}
