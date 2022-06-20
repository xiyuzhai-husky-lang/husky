use crate::*;

use super::impl_expr::ExprTokenConfig;

impl HuskyTraceTime {
    pub fn feature_branch_trace(
        &mut self,
        parent: &Trace,
        indent: Indent,
        branch: Arc<FeatureBranch>,
    ) -> TraceId {
        self.new_trace(
            Some(parent.id()),
            indent,
            TraceVariant::FeatureBranch(branch),
        )
    }

    pub(crate) fn feature_branch_lines(
        &mut self,
        indent: Indent,
        branch: &FeatureBranch,
    ) -> Vec<TraceLineRawData> {
        vec![TraceLineRawData {
            idx: 0,
            indent,
            tokens: self.feature_branch_tokens(branch),
        }]
    }

    pub(crate) fn feature_branch_tokens(&mut self, branch: &FeatureBranch) -> Vec<TraceTokenData> {
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

    // pub fn feature_branch_subtraces(
    //     &mut self,
    //     parent: &Trace,
    //     branch: &FeatureBranch,
    // ) -> Vec<TraceId> {
    //     self.feature_lazy_block_subtraces(parent, &branch.block)
    // }
}
