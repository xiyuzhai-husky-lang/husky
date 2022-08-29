use super::*;
use crate::*;

impl<'a, 'eval: 'a> FeatureEvaluator<'a, 'eval> {
    pub(crate) fn eval_lazy_branch(
        &self,
        branch: &FeatureLazyBranch,
    ) -> __VMResult<__Register<'eval>> {
        match branch.variant {
            FeatureLazyBranchVariant::If { ref condition } => {
                if !self.eval_expr(condition)?.downcast_bool() {
                    return Ok(__Register::unreturned());
                }
            }
            FeatureLazyBranchVariant::Elif { ref condition } => {
                if !self.eval_expr(condition)?.downcast_bool() {
                    return Ok(__Register::unreturned());
                }
            }
            FeatureLazyBranchVariant::Else => (),
        }
        self.eval_lazy_block(&branch.block)
    }
}
