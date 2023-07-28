use super::*;
use crate::*;

impl<'a, 'static: 'a> FeatureEvaluator<'a> {
    pub(crate) fn eval_lazy_branch(
        &self,
        branch: &FeatureLazyBranch,
    ) -> __VMResult<__RegularValue> {
        match branch.variant {
            FeatureLazyBranchVariant::If { ref condition } => {
                if !self.eval_expr(condition)?.downcast_bool() {
                    return Ok(__RegularValue::unreturned());
                }
            }
            FeatureLazyBranchVariant::Elif { ref condition } => {
                if !self.eval_expr(condition)?.downcast_bool() {
                    return Ok(__RegularValue::unreturned());
                }
            }
            FeatureLazyBranchVariant::Else => (),
        }
        self.eval_lazy_block(&branch.block)
    }
}
