use std::collections::HashMap;

use check_utils::should;
use vm::{__EvalRef, __EvalResult, __EvalValue, __OwnedValue};
use word::CustomIdentifier;

use super::*;

#[derive(Default, Debug)]
pub struct EvalSheet<'eval> {
    values: Mutex<HashMap<EvalKey<'eval>, __EvalValueResult<'eval>>>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum EvalKey<'eval> {
    Feature(FeaturePtr),
    StructDerivedField {
        parent: __EvalRef<'eval>,
        field_ident: CustomIdentifier,
    },
}

unsafe impl<'eval> Send for EvalKey<'eval> {}

unsafe impl<'eval> Sync for EvalKey<'eval> {}

impl<'eval> EvalSheet<'eval> {
    pub(crate) fn cached_value(
        &self,
        eval_key: EvalKey<'eval>,
    ) -> Option<__EvalValueResult<'eval>> {
        self.values
            .lock()
            .unwrap()
            .get(&eval_key)
            .map(|v| unsafe { share_cached(v) })
    }

    pub(crate) fn try_cache(
        &self,
        eval_key: EvalKey<'eval>,
        mut value: __EvalValueResult<'eval>,
    ) -> __EvalValueResult<'eval> {
        let mut values = self.values.lock().unwrap();
        if !values.contains_key(&eval_key) {
            let result = unsafe { cache_raw_eval_value(&mut value) };
            assert!(values.insert(eval_key, value).is_none());
            result
        } else {
            value
        }
    }

    pub(crate) fn cache(
        &self,
        eval_key: EvalKey<'eval>,
        mut value: __EvalValueResult<'eval>,
    ) -> __EvalValueResult<'eval> {
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

unsafe fn cache_raw_eval_value<'eval>(
    raw: &mut __EvalValueResult<'eval>,
) -> __EvalValueResult<'eval> {
    match raw {
        Ok(value) => match value {
            __EvalValue::Copyable(value) => {
                *raw = Ok(__EvalValue::Owned(
                    value.any_ref().__clone_into_box_dyn().into(),
                ))
            }
            _ => (),
        },
        Err(error) => (),
    }
    share_cached(raw)
}

unsafe fn share_cached<'eval>(cached: &__EvalValueResult<'eval>) -> __EvalValueResult<'eval> {
    match cached {
        Ok(value) => Ok(match value {
            __EvalValue::Copyable(value) => panic!(),
            __EvalValue::Owned(value) => __EvalValue::EvalRef(__EvalRef(&*value.any_ptr())),
            __EvalValue::EvalRef(value) => __EvalValue::EvalRef(*value),
            __EvalValue::EvalPure(value) => __EvalValue::EvalPure(value.clone()),
            __EvalValue::Undefined => __EvalValue::Undefined,
            __EvalValue::Unreturned => __EvalValue::Unreturned,
        }),
        Err(error) => Err(error.clone()),
    }
}

pub trait HasFeatureSheet<'cache> {
    fn feature_sheet(&self, idx: usize) -> &EvalSheet<'cache>;
}
