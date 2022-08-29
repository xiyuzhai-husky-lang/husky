use crate::*;
use std::{
    ffi::c_void,
    panic::{RefUnwindSafe, UnwindSafe},
};

pub trait __EvalContext<'eval>: RefUnwindSafe + UnwindSafe {
    fn entity_uid(&self, entity_route_text: &str) -> u64;

    fn opt_cached_lazy_field(
        &self,
        this: *const c_void,
        uid: u64,
    ) -> Option<__VMResult<__Register<'eval>>>;

    fn cache_feature(
        &self,
        feature: *const c_void,
        value: __VMResult<__Register<'eval>>,
    ) -> __VMResult<__Register<'eval>>;

    fn opt_cached_feature(&self, feature: *const c_void) -> Option<__VMResult<__Register<'eval>>>;

    fn cache_lazy_field(
        &self,
        this: *const c_void,
        uid: u64,
        value: __VMResult<__Register<'eval>>,
    ) -> __VMResult<__Register<'eval>>;

    fn feature_ptr(&self, feature_route_text: &str) -> *const c_void;

    fn eval_feature_from_uid(&self, feature_uid: u64) -> __VMResult<__Register<'eval>>;

    fn target_input(&self) -> &__Register<'eval>;
}

#[macro_export]
macro_rules! feature_ptr {
    ($ctx: ident, $text: expr) => {{
        unsafe {
            static mut __OPT_FEATURE_PTR: Option<*const std::ffi::c_void> = None;
            if let Some(__feature_ptr) = __OPT_FEATURE_PTR {
                __feature_ptr
            } else {
                let __feature_ptr = $ctx.feature_ptr($text);
                __OPT_FEATURE_PTR = Some(__feature_ptr);
                __feature_ptr
            }
        }
    }};
}

#[macro_export]
macro_rules! entity_uid {
    ($ctx: ident, $text: expr) => {{
        unsafe {
            static mut __OPT_ENTITY_UID: Option<u64> = None;
            if let Some(__entity_uid) = __OPT_ENTITY_UID {
                __entity_uid
            } else {
                let __entity_uid = $ctx.entity_uid($text);
                __OPT_ENTITY_UID = Some(__entity_uid);
                __entity_uid
            }
        }
    }};
}
