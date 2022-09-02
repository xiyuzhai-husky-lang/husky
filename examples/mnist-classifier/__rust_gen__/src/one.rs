use crate::*;

pub(crate) fn one_fermi_match_others<'eval>(
    __ctx: &dyn __EvalContext<'eval>,
) -> &'eval Vec<&'eval crate::line_segment_sketch::concave_component::ConcaveComponent<'eval>> {
    let __feature = feature_ptr!(__ctx, "mnist_classifier::one::one_fermi_match_others");
    if let Some(__result) = __ctx.opt_cached_feature(__feature) {
        return __result
            .unwrap()
            .downcast_eval_ref(&__registration__::__VEC_REF_CONCAVE_COMPONENT_VTABLE);
    }
    return __ctx
        .cache_feature(
            __feature,
            Ok(__Register::new_eval_ref::<
                Vec<&'eval crate::line_segment_sketch::concave_component::ConcaveComponent<'eval>>,
            >(
                &(one_fermi_match(__ctx).others),
                &__registration__::__VEC_REF_CONCAVE_COMPONENT_VTABLE,
            )
            .into()),
        )
        .unwrap()
        .downcast_eval_ref(&__registration__::__VEC_REF_CONCAVE_COMPONENT_VTABLE);
}
pub(crate) fn one_fermi_match<'eval>(
    __ctx: &dyn __EvalContext<'eval>,
) -> &'eval crate::fermi::FermiMatchResult<'eval> {
    let __feature = feature_ptr!(__ctx, "mnist_classifier::one::one_fermi_match");
    if let Some(__result) = __ctx.opt_cached_feature(__feature) {
        return __result
            .unwrap()
            .downcast_eval_ref(&__registration__::__FERMI_MATCH_RESULT_VTABLE);
    }
    return __ctx
        .cache_feature(
            __feature,
            Ok(__Register::new_box::<crate::fermi::FermiMatchResult<'eval>>(crate::fermi::fermi_match(&crate::major::major_concave_components(__ctx), &vec![ThickFp::__new_base(downmost as fn(&'static crate::line_segment_sketch::concave_component::ConcaveComponent<'static>)->Option<f32>), ThickFp::__new_base(upmost as fn(&'static crate::line_segment_sketch::concave_component::ConcaveComponent<'static>)->Option<f32>), ThickFp::__new_base(downmost as fn(&'static crate::line_segment_sketch::concave_component::ConcaveComponent<'static>)->Option<f32>)], __ctx), &__registration__::__FERMI_MATCH_RESULT_VTABLE)),
        )
        .unwrap()
        .downcast_eval_ref(&__registration__::__FERMI_MATCH_RESULT_VTABLE);
}
pub(crate) fn upmost<'eval>(
    cc: &'eval crate::line_segment_sketch::concave_component::ConcaveComponent<'eval>,
) -> Option<f32> {
    let dp = cc.displacement();
    normal_require!(dp.y > 0f32);
    return Some(dp.y);
}
pub(crate) fn downmost<'eval>(
    cc: &'eval crate::line_segment_sketch::concave_component::ConcaveComponent<'eval>,
) -> Option<f32> {
    let dp = cc.displacement();
    normal_require!(dp.y <= 0f32);
    return Some(-dp.y);
}
