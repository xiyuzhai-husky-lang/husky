use crate::*;
use husky_ast::AstQueryGroup;
pub use vm::EntityUid as __EntityUid;
use vm::{__AnyValue, __AnyValueDyn, __EvalContext, __EvalRef, __EvalValue};
use wild_utils::wild_arb_ref;

#[macro_export]
macro_rules! feature_ptr {
    ($ctx: ident, $text: expr) => {{
        unsafe {
            static mut __OPT_FEATURE_PTR: Option<__FeaturePtr> = None;
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
            static mut __OPT_ENTITY_UID: Option<__EntityUid> = None;
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
