use crate::*;
use check_utils::should_eq;
use vm::*;

use super::FeatureEvaluator;

impl<'a, 'eval: 'a> FeatureEvaluator<'a, 'eval> {
    pub(crate) fn eval_feature_repr(&mut self, repr: &FeatureRepr) -> EvalValueResult<'eval> {
        let result = match repr {
            FeatureRepr::Value { value, .. } => Ok(EvalValue::EvalRef(value.short())),
            FeatureRepr::Expr(expr) => self.eval_feature_expr(expr),
            FeatureRepr::LazyBlock(block) => self.eval_feature_lazy_block(block),
            FeatureRepr::FuncBlock(block) => self.eval_feature_func_block(block),
            FeatureRepr::ProcBlock(_) => todo!(),
        };
        if let Ok(ref value) = result {
            should_eq!(value.any_ref().ty_dyn(), repr.ty())
        }
        result
    }

    pub(crate) fn eval_feature_repr_cached(
        &mut self,
        repr: &FeatureRepr,
    ) -> EvalValueResult<'eval> {
        let eval_key = EvalKey::Feature(repr.feature());
        if let Some(result) = self.sheet.cached_value(eval_key) {
            if let Ok(ref value) = result {
                should_eq!(value.any_ref().ty_dyn(), repr.ty())
            }
            result
        } else {
            let result = self.eval_feature_repr(repr);
            self.sheet.try_cache(eval_key, result)
        }
    }
}
