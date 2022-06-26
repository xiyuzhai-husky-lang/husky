use super::*;

impl HuskyTraceTime {
    pub(crate) fn feature_branch_tokens(
        &mut self,
        branch: &FeatureLazyBranch,
    ) -> Vec<TraceTokenData> {
        match branch.variant {
            FeatureBranchVariant::If { ref condition } => {
                let mut tokens = vec![keyword!("if ")];
                tokens.extend(self.feature_expr_tokens(condition, ExprTokenConfig::branch()));
                tokens
            }
            FeatureBranchVariant::Elif { ref condition } => {
                let mut tokens = vec![keyword!("elif ")];
                tokens.extend(self.feature_expr_tokens(condition, ExprTokenConfig::branch()));
                tokens
            }
            FeatureBranchVariant::Else => vec![keyword!("else ")],
        }
    }
}
