use crate::*;
use std::panic::{RefUnwindSafe, UnwindSafe};

pub trait __EvalContext<'eval>: RefUnwindSafe + UnwindSafe {
    fn entity_uid(&self, entity_route_text: &str) -> usize;

    fn opt_cached_lazy_field(
        &self,
        this: &'eval (),
        uid: usize,
    ) -> Option<__VMResult<__Register<'eval>>>;

    fn cache_feature(
        &self,
        feature: *const (),
        value: __VMResult<__Register<'eval>>,
    ) -> __VMResult<__Register<'eval>>;

    fn opt_cached_feature(&self, feature: *const ()) -> Option<__VMResult<__Register<'eval>>>;

    fn cache_lazy_field(
        &self,
        this: &'eval (),
        uid: usize,
        value: __VMResult<__Register<'eval>>,
    ) -> __VMResult<__Register<'eval>>;

    fn get_feature_ptr(&self, feature_route_text: &str) -> *const ();

    fn eval_feature_from_uid(&self, feature_uid: usize) -> __VMResult<__Register<'eval>>;
}
