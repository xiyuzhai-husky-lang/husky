use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ConnectedComponent {
    pub(crate) mask: domains::ml::datasets::cv::mnist::BinaryImage28,
}

impl ConnectedComponent {
    pub(crate) fn __call__(mask: domains::ml::datasets::cv::mnist::BinaryImage28) -> Self {
        Self { mask }
    }
    pub(crate) fn raw_contours<'eval>(
        &'eval self,
        __ctx: &dyn __EvalContext<'eval>,
    ) -> &'eval Vec<crate::raw_contour::RawContour<'eval>> {
        let __uid = entity_uid!(
            __ctx,
            "mnist_classifier::connected_component::ConnectedComponent::raw_contours"
        );
        if let Some(__result) = __ctx.opt_cached_lazy_field(self as *const _ as *const (), __uid) {
            return __result
                .unwrap()
                .downcast_eval_ref(&__registration__::__VEC_RAW_CONTOUR_VTABLE);
        }
        return __ctx
            .cache_lazy_field(
                self as *const _ as *const (),
                __uid,
                Ok(__Register::new_box::<
                    Vec<crate::raw_contour::RawContour<'eval>>,
                >(
                    crate::raw_contour::find_raw_contours(self),
                    &__registration__::__VEC_RAW_CONTOUR_VTABLE,
                )),
            )
            .unwrap()
            .downcast_eval_ref(&__registration__::__VEC_RAW_CONTOUR_VTABLE);
    }
}

impl __StaticInfo for ConnectedComponent {
    type __StaticSelf = ConnectedComponent;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        "mnist_classifier::connected_component::ConnectedComponent".into()
    }
}

impl<'eval> __Registrable<'eval> for ConnectedComponent {
    unsafe fn __to_register(self) -> __Register<'eval> {
        todo!()
    }
}
pub(crate) fn connected_components<'eval>(
    __ctx: &dyn __EvalContext<'eval>,
) -> &'eval Vec<ConnectedComponent> {
    let __feature = feature_ptr!(
        __ctx,
        "mnist_classifier::connected_component::connected_components"
    );
    if let Some(__result) = __ctx.opt_cached_feature(__feature) {
        return __result
            .unwrap()
            .downcast_eval_ref(&__registration__::__VEC_CONNECTED_COMPONENT_VTABLE);
    }
    return __ctx
        .cache_feature(
            __feature,
            Ok(__Register::new_box::<Vec<ConnectedComponent>>(
                find_connected_components(&__input(__ctx)),
                &__registration__::__VEC_CONNECTED_COMPONENT_VTABLE,
            )),
        )
        .unwrap()
        .downcast_eval_ref(&__registration__::__VEC_CONNECTED_COMPONENT_VTABLE);
}
pub(crate) fn major_connected_component<'eval>(
    __ctx: &dyn __EvalContext<'eval>,
) -> &'eval ConnectedComponent {
    let __feature = feature_ptr!(
        __ctx,
        "mnist_classifier::connected_component::major_connected_component"
    );
    if let Some(__result) = __ctx.opt_cached_feature(__feature) {
        return __result
            .unwrap()
            .downcast_eval_ref(&__registration__::__CONNECTED_COMPONENT_VTABLE);
    }
    return __ctx
        .cache_feature(
            __feature,
            Ok(__Register::new_eval_ref::<ConnectedComponent>(
                &(connected_components(__ctx)[(0) as usize]),
                &__registration__::__CONNECTED_COMPONENT_VTABLE,
            )
            .into()),
        )
        .unwrap()
        .downcast_eval_ref(&__registration__::__CONNECTED_COMPONENT_VTABLE);
}

pub(crate) fn horizontal_extend(a: u32, x: u32) -> u32 {
    let mut y = a & (x | (x << 1) | (x >> 1));
    let mut z = a & (y | (y << 1) | (y >> 1));
    while z != y {
        y = z;
        z = a & (y | (y << 1) | (y >> 1));
    }
    return y;
}

pub(crate) fn find_connected_components(
    img: &domains::ml::datasets::cv::mnist::BinaryImage28,
) -> Vec<ConnectedComponent> {
    let mut result = Vec::<ConnectedComponent>::__call__(vec![]);
    let mut unsearched = img.clone();
    for j in 0..30 {
        while unsearched[(j) as usize] != 0 {
            let a = unsearched[(j) as usize];
            let shift = a.ctz();
            let mut mask = domains::ml::datasets::cv::mnist::BinaryImage28::__call__();
            mask[(j) as usize] = horizontal_extend(a, 1u32 << shift);
            let mut flag = false;
            while !flag {
                flag = true;
                let mut i = j;
                while i < 30 - 1 {
                    let old_row = mask[(i + 1) as usize];
                    let new_row =
                        old_row | horizontal_extend(img[(i + 1) as usize], mask[(i) as usize]);
                    if (0 == new_row) {
                        break;
                    }
                    if old_row != new_row {
                        flag = false;
                        mask[(i + 1) as usize] = new_row;
                    }
                    i += 1;
                }
                while i >= j {
                    let old_row = mask[(i) as usize];
                    let new_row =
                        old_row | horizontal_extend(img[(i) as usize], mask[(i + 1) as usize]);
                    if old_row != new_row {
                        flag = false;
                        mask[(i) as usize] = new_row;
                    }
                    i -= 1;
                }
            }
            for k in j..30 {
                unsearched[(k) as usize] &= (!mask[(k) as usize]);
            }
            result.push(ConnectedComponent::__call__(mask));
        }
    }
    return result;
}
