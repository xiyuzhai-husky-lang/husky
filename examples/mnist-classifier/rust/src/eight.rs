use crate::*;

pub(crate) fn upper_mouth_match<'eval>(
    __ctx: &dyn __EvalContext<'eval>,
) -> &'eval crate::fermi::FermiMatchResult<'eval> {
    let __feature = feature_ptr!(__ctx, "mnist_classifier::eight::upper_mouth_match");
    if let Some(__result) = __ctx.opt_cached_feature(__feature) {
        return __result
            .unwrap()
            .downcast_eval_ref(&__registration__::__FERMI_MATCH_RESULT_VTABLE);
    }
    return __ctx
        .cache_feature(
            __feature,
            Ok(__Register::new_box::<crate::fermi::FermiMatchResult<'eval>>(crate::fermi::fermi_match(&crate::major::major_concave_components(__ctx), &vec![ThickFp::__new_ctx(big_mouth as fn(&'static crate::line_segment_sketch::concave_component::ConcaveComponent<'static>, &dyn __EvalContext<'static>)->Option<f32>)], __ctx), &__registration__::__FERMI_MATCH_RESULT_VTABLE)),
        )
        .unwrap()
        .downcast_eval_ref(&__registration__::__FERMI_MATCH_RESULT_VTABLE);
}
pub(crate) fn big_mouth<'eval>(
    cc: &'eval crate::line_segment_sketch::concave_component::ConcaveComponent<'eval>,
    __ctx: &dyn __EvalContext<'eval>,
) -> Option<f32> {
    if cc.relative_bounding_box(__ctx).ymax() > 0.5f32 {
        normal_require!(cc.strokes.firstx().start.x > cc.strokes.firstx().end.x);
    }
    return Some(cc.relative_bounding_box(__ctx).ymax());
}
