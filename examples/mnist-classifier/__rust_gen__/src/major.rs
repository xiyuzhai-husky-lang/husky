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
