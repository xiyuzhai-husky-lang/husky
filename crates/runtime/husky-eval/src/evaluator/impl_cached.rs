use crate::*;
use husky_vm::*;

use super::ValEvaluator;

impl<'temp> ValEvaluator<'temp> {
    pub(super) fn eval_cached(
        &self,
        eval_key: EvalKey,
        f: impl FnOnce(&Self) -> __VMResult<__RegularValue>,
    ) -> __VMResult<__RegularValue> {
        if let Some(result) = self.sheet.cached_value(eval_key) {
            result
        } else {
            let result = f(self);
            self.sheet.try_cache(eval_key, result)
        }
    }
}
