use crate::*;
use vm::*;

use super::FeatureEvaluator;

impl<'a, 'eval: 'a> FeatureEvaluator<'a, 'eval> {
    pub(super) fn eval_feature_repr(&mut self, repr: &FeatureRepr) -> EvalResult<'eval> {
        msg_once!("ad hoc");
        match repr {
            FeatureRepr::Expr(expr) => self.eval_feature_expr(expr),
            FeatureRepr::LazyBlock(block) => {
                self.eval_feature_block(block, EvalKey::Feature(block.feature))
            }
            FeatureRepr::FuncBlock(_) => todo!(),
            FeatureRepr::ProcBlock(_) => todo!(),
        }
    }
}
