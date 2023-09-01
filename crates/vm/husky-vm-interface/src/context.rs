use crate::*;
use std::{
    ffi::c_void,
    panic::{RefUnwindSafe, UnwindSafe},
};

pub trait __EvalContext {
    fn item_uid(&self, item_route_text: &str) -> u64;

    fn opt_cached_lazy_field(
        &self,
        this: *const c_void,
        uid: u32,
    ) -> Option<VMResult<RegularValue>>;

    fn cache_feature(
        &self,
        feature_raw_id: u32,
        value: VMResult<RegularValue>,
    ) -> VMResult<RegularValue>;

    fn opt_cached_feature(&self, feature_raw_id: u32) -> Option<VMResult<RegularValue>>;

    fn cache_lazy_field(
        &self,
        this: *const c_void,
        uid: u32,
        value: VMResult<RegularValue>,
    ) -> VMResult<RegularValue>;

    fn feature_raw_id(&self, feature_route_text: &str) -> u32;

    fn eval_feature_from_uid(&self, feature_item_uid: u32) -> VMResult<RegularValue>;

    fn target_input(&self) -> &RegularValue;
}

#[macro_export]
macro_rules! feature_ptr {
    ($ctx: ident, $text: expr) => {{
        unsafe {
            static mut __OPT_FEATURE_PTR: Option<usize> = None;
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
macro_rules! item_uid {
    ($ctx: ident, $text: expr) => {{
        unsafe {
            static mut __OPT_ENTITY_UID: Option<u64> = None;
            if let Some(__item_uid) = __OPT_ENTITY_UID {
                __item_uid
            } else {
                let __item_uid = $ctx.item_uid($text);
                __OPT_ENTITY_UID = Some(__item_uid);
                __item_uid
            }
        }
    }};
}
