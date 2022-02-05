use vm::{Conditional, EvalValue, StackValue};

use super::*;

#[derive(Default)]
pub struct FeatureSheet<'eval> {
    values: HashMap<FeaturePtr, EvalValue<'eval, 'eval>>,
}

impl<'eval> FeatureSheet<'eval> {
    pub(super) fn value(
        &mut self,
        feature: FeaturePtr,
        compute_value: impl FnOnce() -> EvalValue<'eval, 'eval>,
    ) -> EvalValue<'eval, 'eval> {
        unsafe { share_cached(self.values.entry(feature).or_insert_with(compute_value)) }
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
                StackValue::Volatile(_) => todo!(),
            }),
            Conditional::Undefined => Conditional::Undefined,
        }),
        Err(error) => Err(error.clone()),
    }
}

pub trait HasFeatureSheet<'cache> {
    fn feature_sheet(&self, idx: usize) -> &FeatureSheet<'cache>;
}
