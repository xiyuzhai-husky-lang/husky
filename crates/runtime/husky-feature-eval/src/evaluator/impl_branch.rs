use super::*;
use crate::*;
use husky_print_utils::{epin, msg_once, p};
use husky_vm::{__RegisterDataKind, eval_fast};

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
