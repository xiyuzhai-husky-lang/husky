use crate::*;
use std::{
    marker::PhantomData,
    panic::{RefUnwindSafe, UnwindSafe},
};

pub trait __EvalContext<'eval>: RefUnwindSafe + UnwindSafe {
    fn entity_uid(&self, entity_route_text: &str) -> EntityUid;

    fn opt_cached_lazy_field(
        &self,
        this: &'eval dyn __AnyValueDyn<'eval>,
        uid: EntityUid,
    ) -> Option<__EvalValueResult<'eval>>;

    fn cache_feature(
        &self,
        feature: *const (),
        value: __EvalValueResult<'eval>,
    ) -> __EvalValueResult<'eval>;

    fn opt_cached_feature(&self, feature: *const ()) -> Option<__EvalValueResult<'eval>>;

    fn cache_lazy_field(
        &self,
        this: &'eval dyn __AnyValueDyn<'eval>,
        uid: EntityUid,
        value: __EvalValueResult<'eval>,
    ) -> __EvalValueResult<'eval>;

    fn get_feature_ptr(&self, feature_route_text: &str) -> *const ();
}

// impl<'eval> dyn __EvalContext<'eval> {
//     fn opt_cached_lazy_field_downcast_ref<T>(
//         &self,
//         this: &'eval dyn __AnyValueDyn<'eval>,
//         uid: EntityUid,
//     ) -> Option<__EvalResult<&'eval T>>
//     where
//         T: __AnyValue<'eval>,
//     {
//         self.opt_cached_lazy_field(this, uid)
//             .map(|result| result.map(|v| v.eval_ref().0.__downcast_ref()))
//     }

//     fn opt_cached_feature_downcast_ref<T>(
//         &self,
//         feature: *const (),
//     ) -> Option<__EvalResult<&'eval T>>
//     where
//         T: __AnyValue<'eval>,
//     {
//         self.opt_cached_feature(feature)
//             .map(|result| result.map(|v| v.eval_ref().0.__downcast_ref()))
//     }
// }
