use crate::*;
use husky_check_utils::should_eq;
use husky_entity_route::EntityRoutePtr;
use husky_entity_semantics::{CallFormSource, EntityDefnVariant};
use husky_feature_gen::*;
use husky_lazy_semantics::LazyStmt;
use husky_print_utils::{epin, msg_once, p};
use husky_trace_protocol::VisualData;
use husky_vm::__Linkage;
use husky_vm::*;
use husky_word::IdentPairDict;
use std::{iter::zip, sync::Arc};

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
