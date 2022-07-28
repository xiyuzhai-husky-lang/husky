use super::FeatureEvaluator;
use crate::*;
use husky_check_utils::{should, should_eq};
use husky_compile_time::*;
use husky_print_utils::{epin, p};
use vm::*;

impl<'a, 'eval: 'a> FeatureEvaluator<'a, 'eval> {
    pub(crate) fn eval_feature_repr(
        &mut self,
        repr: &FeatureRepr,
    ) -> __VMResult<__Register<'eval>> {
        let result = match repr {
            FeatureRepr::Value { value, .. } => Ok(value.snapshot()),
            FeatureRepr::Expr(expr) => self.eval_expr(expr),
            FeatureRepr::LazyBlock(block) => self.eval_feature_lazy_block(block),
            FeatureRepr::FuncBlock(block) => self.eval_feature_func_block(block),
            FeatureRepr::ProcBlock(_) => todo!(),
        };
        result
    }

    pub(crate) fn eval_feature_repr_cached(
        &mut self,
        repr: &FeatureRepr,
    ) -> __VMResult<__Register<'eval>> {
        let eval_key = EvalKey::Feature(repr.feature());
        if let Some(result) = self.sheet.cached_value(eval_key) {
            result
        } else {
            let result = self.eval_feature_repr(repr);
            self.sheet.try_cache(eval_key, result)
        }
    }
}
