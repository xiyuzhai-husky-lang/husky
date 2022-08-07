#![allow(warnings)]
pub mod __init__;
pub mod __registration__;
use __husky::root::*;

fn __input<'a, 'eval: 'a>(__ctx: &'a dyn __EvalContext<'eval>) -> &'a f32 {
    unsafe {
        __ctx
            .target_input()
            .downcast_temp_ref(&__registration__::__F32_VTABLE)
    }
}
pub(crate) fn branch_in_func_feature<'eval>(__ctx: &dyn __EvalContext<'eval>) -> &'eval i32 {
    let __feature = feature_ptr!(__ctx, "control_flow::branch_in_func_feature");
    if let Some(__result) = __ctx.opt_cached_feature(__feature) {
        return __result
            .unwrap()
            .downcast_eval_ref(&__registration__::__I32_VTABLE);
    }
    if 3f32 > 0f32 {
        return __ctx
            .cache_feature(
                __feature,
                Ok(__Register::new_box(1, &__registration__::__I32_VTABLE)),
            )
            .unwrap()
            .downcast_eval_ref(&__registration__::__I32_VTABLE);
    } else {
        return __ctx
            .cache_feature(
                __feature,
                Ok(__Register::new_box(2, &__registration__::__I32_VTABLE)),
            )
            .unwrap()
            .downcast_eval_ref(&__registration__::__I32_VTABLE);
    }
}
pub(crate) fn match_in_func_feature<'eval>(__ctx: &dyn __EvalContext<'eval>) -> &'eval i32 {
    let __feature = feature_ptr!(__ctx, "control_flow::match_in_func_feature");
    if let Some(__result) = __ctx.opt_cached_feature(__feature) {
        return __result
            .unwrap()
            .downcast_eval_ref(&__registration__::__I32_VTABLE);
    }
    if 3f32 > 0f32 {
        return __ctx
            .cache_feature(
                __feature,
                Ok(__Register::new_box(1, &__registration__::__I32_VTABLE)),
            )
            .unwrap()
            .downcast_eval_ref(&__registration__::__I32_VTABLE);
    } else {
        return __ctx
            .cache_feature(
                __feature,
                Ok(__Register::new_box(2, &__registration__::__I32_VTABLE)),
            )
            .unwrap()
            .downcast_eval_ref(&__registration__::__I32_VTABLE);
    }
}
pub(crate) fn branch_in_func_function() -> i32 {
    if 3f32 > 0f32 {
        return 1;
    } else {
        return 2;
    }
}
pub(crate) fn match_in_func_function() -> i32 {
    match 1u32 {
        2u32 => {
            return 1;
        }
        1 | 0 => {
            return 1;
        }
        _ => {
            return 3;
        }
    }
}
