use std::collections::HashMap;

use vm::{EvalValue, __EvalRef, __EvalResult, __OwnedValue};
use word::CustomIdentifier;

use super::*;

#[derive(Default, Debug)]
pub struct EvalSheet<'eval> {
    values: Mutex<HashMap<EvalKey<'eval>, EvalValueResult<'eval>>>,
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
    pub(crate) fn cached_value(&self, eval_key: EvalKey<'eval>) -> Option<EvalValueResult<'eval>> {
        self.values
            .lock()
            .unwrap()
            .get(&eval_key)
            .map(|v| unsafe { share_cached(v) })
    }

    pub(crate) fn try_cache(
        &self,
        eval_key: EvalKey<'eval>,
        mut value: EvalValueResult<'eval>,
    ) -> EvalValueResult<'eval> {
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
        mut value: EvalValueResult<'eval>,
    ) -> EvalValueResult<'eval> {
        let result = unsafe { cache_raw_eval_value(&mut value) };
        assert!(self
            .values
            .lock()
            .unwrap()
            .insert(eval_key, value)
            .is_none());
        result
    }
}

unsafe fn cache_raw_eval_value<'eval>(raw: &mut EvalValueResult<'eval>) -> EvalValueResult<'eval> {
    match raw {
        Ok(value) => match value {
            EvalValue::Copyable(value) => {
                *raw = Ok(EvalValue::Owned(
                    value.any_ref().__clone_into_box_dyn().into(),
                ))
            }
            _ => (),
        },
        Err(error) => (),
    }
    share_cached(raw)
}

unsafe fn share_cached<'eval>(cached: &EvalValueResult<'eval>) -> EvalValueResult<'eval> {
    match cached {
        Ok(value) => Ok(match value {
            EvalValue::Copyable(value) => panic!(),
            EvalValue::Owned(value) => EvalValue::EvalRef(__EvalRef(&*value.any_ptr())),
            EvalValue::EvalRef(value) => EvalValue::EvalRef(*value),
            EvalValue::EvalPure(value) => EvalValue::EvalPure(value.clone()),
            EvalValue::Undefined => EvalValue::Undefined,
        }),
        Err(error) => Err(error.clone()),
    }
}

pub trait HasFeatureSheet<'cache> {
    fn feature_sheet(&self, idx: usize) -> &EvalSheet<'cache>;
}
