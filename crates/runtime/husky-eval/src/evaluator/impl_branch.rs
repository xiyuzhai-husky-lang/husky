use super::*;
use crate::*;

impl<'a> ValEvaluator<'a> {
    pub(crate) fn eval_lazy_branch(&self, branch: ValBranch) -> __VMResult<__RegularValue> {
        match branch.variant {
            FeatureLazyBranchVariant::If { ref condition } => {
                if !unsafe { self.eval_expr(condition)?.downcast_bool() } {
                    return Ok(__RegularValue::unreturned());
                }
            }
            FeatureLazyBranchVariant::Elif { ref condition } => {
                if !unsafe { self.eval_expr(condition)?.downcast_bool() } {
                    return Ok(__RegularValue::unreturned());
                }
            }
            FeatureLazyBranchVariant::Else => (),
        }
        self.eval_lazy_block(&branch.block)
    }
}
