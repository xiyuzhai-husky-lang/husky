use vm::{EvalResult, EvalValue};

use super::*;

#[derive(Default, Debug)]
pub struct FeatureSheet<'eval> {
    values: HashMap<FeaturePtr, EvalResult<'eval>>,
}

impl<'eval> FeatureSheet<'eval> {
    pub(crate) fn cached_value(&self, feature: FeaturePtr) -> Option<EvalResult<'eval>> {
        self.values
            .get(&feature)
            .map(|v| unsafe { share_cached(v) })
    }

    pub(crate) fn cache(
        &mut self,
        feature: FeaturePtr,
        value: EvalResult<'eval>,
    ) -> EvalResult<'eval> {
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

unsafe fn share_cached<'eval>(cached: &EvalResult<'eval>) -> EvalResult<'eval> {
    Ok(match cached {
        Ok(value) => match value {
            EvalValue::Primitive(value) => EvalValue::Primitive(*value),
            EvalValue::Boxed(value) => EvalValue::GlobalRef(&*value.pointer()),
            EvalValue::GlobalRef(_) => todo!(),
            EvalValue::GlobalPure(_) => todo!(),
            EvalValue::Undefined => EvalValue::Undefined,
        },
        Err(error) => Err(error.clone())?,
    })
}

pub trait HasFeatureSheet<'cache> {
    fn feature_sheet(&self, idx: usize) -> &FeatureSheet<'cache>;
}
