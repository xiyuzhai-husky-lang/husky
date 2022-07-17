use crate::*;
use vm::{__EvalContext, __EvalValue};

pub fn __cache_feature<'eval, T>(
    __ctx: &__EvalContext<'eval>,
    feature: FeaturePtr,
    value: __EvalValue<'eval>,
) -> &'eval T {
    todo!()
}

#[macro_export]
macro_rules! feature_ptr {
    ($ctx: ident, $text: expr) => {{
        unsafe {
            static mut __OPT_FEATURE_PTR: Option<__FeaturePtr> = None;
            if let Some(__feature_ptr) = __OPT_FEATURE_PTR {
                __feature_ptr
            } else {
                let __feature_ptr = __get_feature_ptr($ctx, $text);
                __OPT_FEATURE_PTR = Some(__feature_ptr);
                __feature_ptr
            }
        }
    }};
}

pub fn __get_feature_ptr<'eval>(
    __ctx: &__EvalContext<'eval>,
    feature_route_text: &str,
) -> FeaturePtr {
    todo!()
}
