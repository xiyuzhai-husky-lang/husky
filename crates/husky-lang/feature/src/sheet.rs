use vm::{Conditional, EvalValue, StackValue};

use super::*;

#[derive(Default, Debug)]
pub struct FeatureSheet<'eval> {
    values: HashMap<FeaturePtr, EvalValue<'eval, 'eval>>,
}

impl<'eval> FeatureSheet<'eval> {
    pub(crate) fn cached_value(&self, feature: FeaturePtr) -> Option<EvalValue<'eval, 'eval>> {
        self.values
            .get(&feature)
            .map(|v| unsafe { share_cached(v) })
    }

    pub(crate) fn cache(
        &mut self,
        feature: FeaturePtr,
        value: EvalValue<'eval, 'eval>,
    ) -> EvalValue<'eval, 'eval> {
        let result = unsafe { share_cached(&value) };
        assert!(self.values.insert(feature, value).is_none());
        result
    }

    // pub(super) fn value(
    //     &mut self,
    //     feature: FeaturePtr,
    //     compute_value: impl FnOnce() -> EvalValue<'eval, 'eval>,
    // ) -> EvalValue<'eval, 'eval> {
    //     unsafe { share_cached(self.values.entry(feature).or_insert_with(compute_value)) }
    // }
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
