use super::FeatureEvaluator;
use crate::*;
use husky_vm::*;

impl<'a, 'static: 'a> FeatureEvaluator<'a> {
    pub(crate) fn eval_feature_repr(&self, repr: &ValRepr) -> __VMResult<__RegularValue> {
        let result = match repr {
            ValRepr::Value { value, .. } => Ok(value.snapshot()),
            ValRepr::LazyExpr(expr) => self.eval_expr(expr),
            ValRepr::LazyBody(block) => self.eval_lazy_block(block),
            ValRepr::FuncBody(block) => self.eval_func_block(block),
            ValRepr::ProcBody(block) => self.eval_proc_block(block),
            ValRepr::TargetInput { .. } => Ok(self.target_input.bind_temp_ref()),
        };
        result
    }

    pub(crate) fn eval_feature_repr_cached(&self, repr: &ValRepr) -> __VMResult<__RegularValue> {
        let eval_key = EvalKey::Feature(repr.feature());
        if let Some(result) = self.sheet.cached_value(eval_key) {
            result
        } else {
            let result = self.eval_feature_repr(repr);
            match repr {
                ValRepr::TargetInput { .. } => result, // ad hoc
                _ => self.sheet.try_cache(eval_key, result),
            }
        }
    }
}
