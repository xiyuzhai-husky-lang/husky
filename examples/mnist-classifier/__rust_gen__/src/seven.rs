use crate::*;

pub(crate) fn special_seven_match<'eval>(
    __ctx: &dyn __EvalContext<'eval>,
) -> &'eval crate::fermi::FermiMatchResult<'eval> {
    let __feature = feature_ptr!(__ctx, "mnist_classifier::seven::special_seven_match");
    if let Some(__result) = __ctx.opt_cached_feature(__feature) {
        return __result
            .unwrap()
            .downcast_eval_ref(&__registration__::__FERMI_MATCH_RESULT_VTABLE);
    }
    return __ctx
        .cache_feature(
            __feature,
            Ok(__Register::new_box::<crate::fermi::FermiMatchResult<'eval>>(crate::fermi::fermi_match(&crate::major::major_concave_components(__ctx), &vec![ThickFp::__new_ctx(leftupcc_pattern as fn(&'static crate::line_segment_sketch::concave_component::ConcaveComponent<'static>, &dyn __EvalContext<'static>)->Option<f32>)], __ctx), &__registration__::__FERMI_MATCH_RESULT_VTABLE)),
        )
        .unwrap()
        .downcast_eval_ref(&__registration__::__FERMI_MATCH_RESULT_VTABLE);
}
pub(crate) fn leftupcc_pattern<'eval>(
    cc: &'eval crate::line_segment_sketch::concave_component::ConcaveComponent<'eval>,
    __ctx: &dyn __EvalContext<'eval>,
) -> Option<f32> {
    let dp = cc.displacement();
    normal_require!(dp.y < 0f32);
    normal_require!(cc.relative_bounding_box(__ctx).ymax() > 0.3f32);
    let ang = cc.start_tangent().angle(true);
    normal_require!(ang < 30f32);
    return Some(ang);
}
