use crate::*;
use vm::*;

use super::FeatureEvaluator;

impl<'a, 'eval: 'a> FeatureEvaluator<'a, 'eval> {
    pub(super) fn eval_feature_repr(
        &mut self,
        repr: &FeatureRepr,
        eval_key: EvalKey<'eval>,
    ) -> EvalResult<'eval> {
        msg_once!("ad hoc");
        match repr {
            FeatureRepr::Expr(expr) => todo!("use eval key"),
            // self.eval_feature_expr(expr),
            FeatureRepr::LazyBlock(block) => self.eval_feature_block(block, eval_key),
            FeatureRepr::FuncBlock(_) => todo!(),
            FeatureRepr::ProcBlock(_) => todo!(),
        }
    }
}
