use std::collections::HashMap;

use husky_check_utils::should;
use husky_word::CustomIdentifier;
use vm::{EntityUid, __Register, __RegisterDataKind, __VMResult};

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
            unsafe { share_cached(&values[&eval_key]) }
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
    match raw {
        Ok(value) => match value.data_kind {
            __RegisterDataKind::PrimitiveValue => *raw = Ok(value.primitive().into_box_register()),
            _ => (),
        },
        Err(error) => (),
    }
    share_cached(raw)
}

unsafe fn share_cached<'eval>(
    cached: &__VMResult<__Register<'eval>>,
) -> __VMResult<__Register<'eval>> {
    match cached {
        Ok(value) => Ok(match value.data_kind {
            __RegisterDataKind::PrimitiveValue => panic!(),
            __RegisterDataKind::Box | __RegisterDataKind::EvalRef => __Register {
                data_kind: __RegisterDataKind::EvalRef,
                opt_data: value.opt_data,
                phantom: std::marker::PhantomData,
            },
            __RegisterDataKind::TempRef => todo!(),
            __RegisterDataKind::TempMut => todo!(),
            __RegisterDataKind::Moved => todo!(),
            __RegisterDataKind::Undefined => todo!(),
            __RegisterDataKind::Unreturned => todo!(),
            // __Register::Copyable(value) => panic!(),
            // __Register::Owned(value) => __Register::EvalRef(__EvalRef(&*value.any_ptr())),
            // __Register::EvalRef(value) => __Register::EvalRef(*value),
            // __Register::EvalPure(value) => __Register::EvalPure(value.clone()),
            // __Register::Undefined => __Register::Undefined,
            // __Register::Unreturned => __Register::Unreturned,
        }),
        Err(error) => Err(error.clone()),
    }
}

pub trait HasFeatureSheet<'cache> {
    fn feature_sheet(&self, idx: usize) -> &EvalSheet<'cache>;
}
