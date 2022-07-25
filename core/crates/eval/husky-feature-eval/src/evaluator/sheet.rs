use std::collections::HashMap;

use husky_check_utils::should;
use husky_word::CustomIdentifier;
use vm::{EntityUid, __Register, __VMResult};

use super::*;

#[derive(Default, Debug)]
pub struct EvalSheet<'eval> {
    values: Mutex<HashMap<EvalKey<'eval>, __VMResult<__Register<'eval>>>>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum EvalKey<'eval> {
    Feature(FeaturePtr),
    StructDerivedField {
        parent: *const (dyn __RegistrableDyn + 'eval),
        field_uid: EntityUid,
    },
}

unsafe impl<'eval> Send for EvalKey<'eval> {}

unsafe impl<'eval> Sync for EvalKey<'eval> {}

impl<'eval> EvalSheet<'eval> {
    pub(crate) fn cached_value(
        &self,
        eval_key: EvalKey<'eval>,
    ) -> Option<__VMResult<__Register<'eval>>> {
        self.values
            .lock()
            .unwrap()
            .get(&eval_key)
            .map(|v| unsafe { share_cached(v) })
    }

    pub(crate) fn try_cache(
        &self,
        eval_key: EvalKey<'eval>,
        mut value: __VMResult<__Register<'eval>>,
    ) -> __VMResult<__Register<'eval>> {
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
        mut value: __VMResult<__Register<'eval>>,
    ) -> __VMResult<__Register<'eval>> {
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
    raw: &mut __VMResult<__Register<'eval>>,
) -> __VMResult<__Register<'eval>> {
    todo!()
    // match raw {
    //     Ok(value) => match value {
    //         __Register::Copyable(value) => {
    //             *raw = Ok(__Register::Owned(
    //                 value.any_ref().__clone_into_box_dyn().into(),
    //             ))
    //         }
    //         _ => (),
    //     },
    //     Err(error) => (),
    // }
    // share_cached(raw)
}

unsafe fn share_cached<'eval>(
    cached: &__VMResult<__Register<'eval>>,
) -> __VMResult<__Register<'eval>> {
    todo!()
    // match cached {
    //     Ok(value) => Ok(match value {
    //         __Register::Copyable(value) => panic!(),
    //         __Register::Owned(value) => __Register::EvalRef(__EvalRef(&*value.any_ptr())),
    //         __Register::EvalRef(value) => __Register::EvalRef(*value),
    //         __Register::EvalPure(value) => __Register::EvalPure(value.clone()),
    //         __Register::Undefined => __Register::Undefined,
    //         __Register::Unreturned => __Register::Unreturned,
    //     }),
    //     Err(error) => Err(error.clone()),
    // }
}

pub trait HasFeatureSheet<'cache> {
    fn feature_sheet(&self, idx: usize) -> &EvalSheet<'cache>;
}
