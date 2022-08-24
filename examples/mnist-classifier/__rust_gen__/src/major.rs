use crate::*;

pub(crate) fn connected_components<'eval>(
    __ctx: &dyn __EvalContext<'eval>,
) -> &'eval Vec<crate::connected_component::ConnectedComponent> {
    let __feature = feature_ptr!(__ctx, "mnist_classifier::major::connected_components");
    if let Some(__result) = __ctx.opt_cached_feature(__feature) {
        return __result
            .unwrap()
            .downcast_eval_ref(&__registration__::__VEC_CONNECTED_COMPONENT_VTABLE);
    }
    return __ctx
        .cache_feature(
            __feature,
            Ok(__Register::new_box::<
                Vec<crate::connected_component::ConnectedComponent>,
            >(
                crate::connected_component::find_connected_components(&__input(__ctx)),
                &__registration__::__VEC_CONNECTED_COMPONENT_VTABLE,
            )),
        )
        .unwrap()
        .downcast_eval_ref(&__registration__::__VEC_CONNECTED_COMPONENT_VTABLE);
}
pub(crate) fn major_connected_component<'eval>(
    __ctx: &dyn __EvalContext<'eval>,
) -> &'eval crate::connected_component::ConnectedComponent {
    let __feature = feature_ptr!(__ctx, "mnist_classifier::major::major_connected_component");
    if let Some(__result) = __ctx.opt_cached_feature(__feature) {
        return __result
            .unwrap()
            .downcast_eval_ref(&__registration__::__CONNECTED_COMPONENT_VTABLE);
    }
    return __ctx
        .cache_feature(
            __feature,
            Ok(
                __Register::new_eval_ref::<crate::connected_component::ConnectedComponent>(
                    &(connected_components(__ctx)[(0) as usize]),
                    &__registration__::__CONNECTED_COMPONENT_VTABLE,
                )
                .into(),
            ),
        )
        .unwrap()
        .downcast_eval_ref(&__registration__::__CONNECTED_COMPONENT_VTABLE);
}
pub(crate) fn major_raw_contour<'eval>(
    __ctx: &dyn __EvalContext<'eval>,
) -> &'eval crate::raw_contour::RawContour<'eval> {
    let __feature = feature_ptr!(__ctx, "mnist_classifier::major::major_raw_contour");
    if let Some(__result) = __ctx.opt_cached_feature(__feature) {
        return __result
            .unwrap()
            .downcast_eval_ref(&__registration__::__RAW_CONTOUR_VTABLE);
    }
    return __ctx
        .cache_feature(
            __feature,
            Ok(
                __Register::new_eval_ref::<crate::raw_contour::RawContour<'eval>>(
                    &(major_connected_component(__ctx).raw_contours(__ctx)[(0) as usize]),
                    &__registration__::__RAW_CONTOUR_VTABLE,
                )
                .into(),
            ),
        )
        .unwrap()
        .downcast_eval_ref(&__registration__::__RAW_CONTOUR_VTABLE);
}
pub(crate) fn major_line_segment_sketch<'eval>(
    __ctx: &dyn __EvalContext<'eval>,
) -> &'eval crate::line_segment_sketch::LineSegmentSketch<'eval> {
    let __feature = feature_ptr!(__ctx, "mnist_classifier::major::major_line_segment_sketch");
    if let Some(__result) = __ctx.opt_cached_feature(__feature) {
        return __result
            .unwrap()
            .downcast_eval_ref(&__registration__::__LINE_SEGMENT_SKETCH_VTABLE);
    }
    return __ctx
        .cache_feature(
            __feature,
            Ok(
                __Register::new_eval_ref::<crate::line_segment_sketch::LineSegmentSketch<'eval>>(
                    &(major_raw_contour(__ctx).line_segment_sketch(__ctx)),
                    &__registration__::__LINE_SEGMENT_SKETCH_VTABLE,
                )
                .into(),
            ),
        )
        .unwrap()
        .downcast_eval_ref(&__registration__::__LINE_SEGMENT_SKETCH_VTABLE);
}
pub(crate) fn major_concave_components<'eval>(
    __ctx: &dyn __EvalContext<'eval>,
) -> &'eval Vec<crate::line_segment_sketch::concave_component::ConcaveComponent<'eval>> {
    let __feature = feature_ptr!(__ctx, "mnist_classifier::major::major_concave_components");
    if let Some(__result) = __ctx.opt_cached_feature(__feature) {
        return __result
            .unwrap()
            .downcast_eval_ref(&__registration__::__VEC_CONCAVE_COMPONENT_VTABLE);
    }
    return __ctx
        .cache_feature(
            __feature,
            Ok(__Register::new_eval_ref::<
                Vec<crate::line_segment_sketch::concave_component::ConcaveComponent<'eval>>,
            >(
                &(major_line_segment_sketch(__ctx).concave_components(__ctx)),
                &__registration__::__VEC_CONCAVE_COMPONENT_VTABLE,
            )
            .into()),
        )
        .unwrap()
        .downcast_eval_ref(&__registration__::__VEC_CONCAVE_COMPONENT_VTABLE);
}
