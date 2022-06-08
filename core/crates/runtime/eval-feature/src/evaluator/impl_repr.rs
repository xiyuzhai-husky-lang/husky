use crate::*;
use vm::*;

use super::FeatureEvaluator;

impl<'a, 'eval: 'a> FeatureEvaluator<'a, 'eval> {
    pub(super) fn eval_feature_repr(&mut self, repr: &FeatureRepr) -> EvalResult<'eval> {
        match repr {
            FeatureRepr::LazyExpr(expr) => self.eval_feature_expr(expr),
            FeatureRepr::LazyBlock(block) => self.eval_feature_lazy_block(block),
            FeatureRepr::FuncBlock(block) => self.eval_feature_func_block(block),
            FeatureRepr::ProcBlock(_) => todo!(),
        }
    }
}
