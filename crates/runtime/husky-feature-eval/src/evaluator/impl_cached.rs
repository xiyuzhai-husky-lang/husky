use crate::*;
use husky_vm::*;

use super::FeatureEvaluator;

impl<'temp, 'eval: 'temp> FeatureEvaluator<'temp, 'eval> {
    pub(super) fn eval_cached(
        &mut self,
        eval_key: EvalKey,
        f: impl FnOnce(&mut Self) -> __VMResult<__Register<'eval>>,
    ) -> __VMResult<__Register<'eval>> {
        if let Some(result) = self.sheet.cached_value(eval_key) {
            result
        } else {
            let result = f(self);
            self.sheet.cache(eval_key, result)
        }
    }
}
