use crate::*;
use husky_vm::*;

use super::Evaluator;

impl<'temp> Evaluator<'temp> {
    pub(super) fn eval_cached(
        &self,
        eval_key: EvalKey,
        f: impl FnOnce(&Self) -> __VMResult<RegularValue>,
    ) -> __VMResult<RegularValue> {
        if let Some(result) = self.sheet.cached_value(eval_key) {
            result
        } else {
            let result = f(self);
            self.sheet.try_cache(eval_key, result)
        }
    }
}
