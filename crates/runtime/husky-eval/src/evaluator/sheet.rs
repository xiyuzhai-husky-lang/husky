use std::collections::HashMap;

use husky_check_utils::should;
use husky_vm::{EntityUid, __RegularValue, __VMResult};

use super::*;

#[derive(Default, Debug)]
pub struct EvalSheet {
    values: Mutex<HashMap<EvalKey, __VMResult<__RegularValue>>>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum EvalKey {
    Feature(Val),
    StructDerivedField {
        this: *const c_void,
        field_uid: EntityUid,
    },
}

unsafe impl Send for EvalKey {}

unsafe impl Sync for EvalKey {}

impl EvalSheet {
    pub(crate) fn cached_value(&self, eval_key: EvalKey) -> Option<__VMResult<__RegularValue>> {
        self.values
            .lock()
            .unwrap()
            .get(&eval_key)
            .map(|v| unsafe { share_cached(v) })
    }

    pub(crate) fn try_cache(
        &self,
        eval_key: EvalKey,
        mut value: __VMResult<__RegularValue>,
    ) -> __VMResult<__RegularValue> {
        let mut values = self.values.lock().unwrap();
        if !values.contains_key(&eval_key) {
            let result = unsafe { cache_raw_eval_value(&mut value) };
            assert!(values.insert(eval_key, value).is_none());
            result
        } else {
            unsafe { share_cached(&values[&eval_key]) }
        }
    }

    pub(crate) fn cache(
        &self,
        eval_key: EvalKey,
        mut value: __VMResult<__RegularValue>,
    ) -> __VMResult<__RegularValue> {
        let result = unsafe { cache_raw_eval_value(&mut value) };
        should!(
            self.values
                .lock()
                .unwrap()
                .insert(eval_key, value)
                .is_none(),
            "eval_key {eval_key:?}"
        );
        result
    }
}

unsafe fn cache_raw_eval_value(raw: &mut __VMResult<__RegularValue>) -> __VMResult<__RegularValue> {
    match raw {
        Ok(ref mut value) => value.cache_eval(),
        Err(_) => (),
    }
    share_cached(raw)
}

unsafe fn share_cached(cached: &__VMResult<__RegularValue>) -> __VMResult<__RegularValue> {
    match cached {
        Ok(value) => Ok(value.share_cached()),
        Err(error) => Err(error.clone()),
    }
}

pub trait HasFeatureSheet<'cache> {
    fn feature_sheet(&self, idx: usize) -> &EvalSheet;
}
