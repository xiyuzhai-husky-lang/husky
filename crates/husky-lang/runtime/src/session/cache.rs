use std::{collections::hash_map::Entry, mem::MaybeUninit};

use stdx::sync::ARwLock;
use vm::{Conditional, EvalValue, StackValue};

use super::*;
use crate::*;

#[derive(Default)]
pub struct EvalCache<'eval> {
    storage: ARwLock<HashMap<FeatureId, ARwLock<Option<EvalValue<'eval, 'eval>>>>>,
}

impl<'eval> EvalCache<'eval> {
    pub(super) fn value(
        &self,
        feature: FeatureId,
        compute_value: impl FnOnce() -> EvalValue<'eval, 'eval>,
    ) -> EvalValue<'eval, 'eval> {
        self.storage
            .write(|storage| storage.entry(feature).or_default().clone())
            .write(|maybe_uninit| {
                if let Some(value) = maybe_uninit {
                    unsafe { share_cached(value) }
                } else {
                    *maybe_uninit = Some(compute_value());
                    unsafe { share_cached(maybe_uninit.as_ref().unwrap()) }
                }
            })
    }
}

unsafe fn share_cached<'eval>(cached: &EvalValue<'eval, 'eval>) -> EvalValue<'eval, 'eval> {
    match cached {
        Ok(conditional) => Ok(match conditional {
            Conditional::Defined(value) => Conditional::Defined(match value {
                StackValue::Primitive(value) => StackValue::Primitive(*value),
                StackValue::Boxed(value) => StackValue::GlobalRef(&*value.pointer()),
                StackValue::GlobalRef(_) => todo!(),
                StackValue::Ref(_) => todo!(),
                StackValue::MutRef(_) => todo!(),
            }),
            Conditional::Undefined => Conditional::Undefined,
        }),
        Err(error) => Err(error.clone()),
    }
}
