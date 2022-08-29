use super::FeatureEvaluator;
use crate::*;
use husky_vm::*;

impl<'a, 'eval: 'a> FeatureEvaluator<'a, 'eval> {
    pub(crate) fn eval_feature_repr(&self, repr: &FeatureRepr) -> __VMResult<__Register<'eval>> {
        let result = match repr {
            FeatureRepr::Value { value, .. } => Ok(value.snapshot()),
            FeatureRepr::LazyExpr(expr) => self.eval_expr(expr),
            FeatureRepr::LazyBlock(block) => self.eval_lazy_block(block),
            FeatureRepr::FuncBlock(block) => self.eval_func_block(block),
            FeatureRepr::ProcBlock(_) => todo!(),
            FeatureRepr::TargetInput { .. } => Ok(self.target_input.bind_temp_ref()),
        };
        result
    }

    pub(crate) fn eval_feature_repr_cached(
        &self,
        repr: &FeatureRepr,
    ) -> __VMResult<__Register<'eval>> {
        let eval_key = EvalKey::Feature(repr.feature());
        if let Some(result) = self.sheet.cached_value(eval_key) {
            result
        } else {
            let result = self.eval_feature_repr(repr);
            match repr {
                FeatureRepr::TargetInput { .. } => result, // ad hoc
                _ => self.sheet.try_cache(eval_key, result),
            }
        }
    }
}
