use crate::*;

pub(crate) fn haha<'eval>(__ctx: &dyn __EvalContext<'eval>) -> Option<&'eval i32> {
    let __feature = feature_ptr!(__ctx, "mnist_classifier::one::haha");
    if let Some(__result) = __ctx.opt_cached_feature(__feature) {
        return __result
            .unwrap()
            .downcast_opt_eval_ref(&__registration__::__I32_VTABLE);
    }
    feature_require!(__ctx, __feature, 2 > 0);
    return __ctx
        .cache_feature(
            __feature,
            Ok(__Register::new_box(1, &__registration__::__I32_VTABLE)),
        )
        .unwrap()
        .downcast_opt_eval_ref(&__registration__::__I32_VTABLE);
}
