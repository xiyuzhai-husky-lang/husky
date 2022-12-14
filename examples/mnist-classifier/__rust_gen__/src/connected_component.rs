use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ConnectedComponentDistribution {
    pub(crate) row_start: i32,
    pub(crate) row_end: i32,
    pub(crate) upper_mass: i32,
    pub(crate) lower_mass: i32,
}

impl ConnectedComponentDistribution {
    pub(crate) fn __call__(row_start: i32, row_end: i32, upper_mass: i32, lower_mass: i32) -> Self {
        Self {
            row_start,
            row_end,
            upper_mass,
            lower_mass,
        }
    }
}

impl __StaticInfo for ConnectedComponentDistribution {
    type __StaticSelf = ConnectedComponentDistribution;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        "mnist_classifier::connected_component::ConnectedComponentDistribution".into()
    }

    unsafe fn __transmute_static(self) -> Self::__StaticSelf {
        std::mem::transmute(self)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct EffHoles<'eval> {
    pub(crate) matches: Vec<Option<&'eval crate::raw_contour::RawContour<'eval>>>,
}

impl<'eval> EffHoles<'eval> {
    pub(crate) fn __call__(
        matches: Vec<Option<&'eval crate::raw_contour::RawContour<'eval>>>,
    ) -> Self {
        Self { matches }
    }
}

impl<'eval> __StaticInfo for EffHoles<'eval> {
    type __StaticSelf = EffHoles<'static>;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        "mnist_classifier::connected_component::EffHoles".into()
    }

    unsafe fn __transmute_static(self) -> Self::__StaticSelf {
        std::mem::transmute(self)
    }
}
pub(crate) fn hole_tmpl<'eval>(
    ct: &'eval crate::raw_contour::RawContour<'eval>,
    __ctx: &dyn __EvalContext<'eval>,
) -> Option<f32> {
    let len = *ct.contour_len(__ctx);
    normal_require!(len > 4f32);
    return Some(len + 0f32);
}
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
        if let Some(__result) =
            __ctx.opt_cached_lazy_field(self as *const _ as *const std::ffi::c_void, __uid)
        {
            return __result
                .unwrap()
                .downcast_eval_ref(&__registration__::__VEC_RAW_CONTOUR_VTABLE);
        }
        return __ctx
            .cache_lazy_field(
                self as *const _ as *const std::ffi::c_void,
                __uid,
                Ok(__Register::new_box::<
                    Vec<crate::raw_contour::RawContour<'eval>>,
                >(
                    crate::raw_contour::find_raw_contours(&self),
                    &__registration__::__VEC_RAW_CONTOUR_VTABLE,
                )),
            )
            .unwrap()
            .downcast_eval_ref(&__registration__::__VEC_RAW_CONTOUR_VTABLE);
    }
    pub(crate) fn eff_holes<'eval>(
        &'eval self,
        __ctx: &dyn __EvalContext<'eval>,
    ) -> &'eval EffHoles<'eval> {
        let __uid = entity_uid!(
            __ctx,
            "mnist_classifier::connected_component::ConnectedComponent::eff_holes"
        );
        if let Some(__result) =
            __ctx.opt_cached_lazy_field(self as *const _ as *const std::ffi::c_void, __uid)
        {
            return __result
                .unwrap()
                .downcast_eval_ref(&__registration__::__EFF_HOLES_VTABLE);
        }
        let mut raw_contours = self.raw_contours(__ctx).collect_refs();
        let mut matches = Vec::<Option<&'eval crate::raw_contour::RawContour>>::__call__(vec![]);
        raw_contours.pop_with_largest_opt_f32_copyable(
            ThickFp::__new_ctx(
                hole_tmpl
                    as fn(
                        &'static crate::raw_contour::RawContour<'static>,
                        &dyn __EvalContext<'static>,
                    ) -> Option<f32>,
            ),
            __ctx,
        );
        matches.push(raw_contours.pop_with_largest_opt_f32_copyable(
            ThickFp::__new_ctx(
                hole_tmpl
                    as fn(
                        &'static crate::raw_contour::RawContour<'static>,
                        &dyn __EvalContext<'static>,
                    ) -> Option<f32>,
            ),
            __ctx,
        ));
        matches.push(raw_contours.pop_with_largest_opt_f32_copyable(
            ThickFp::__new_ctx(
                hole_tmpl
                    as fn(
                        &'static crate::raw_contour::RawContour<'static>,
                        &dyn __EvalContext<'static>,
                    ) -> Option<f32>,
            ),
            __ctx,
        ));
        __ctx
            .cache_lazy_field(
                self as *const _ as *const std::ffi::c_void,
                __uid,
                Ok(__Register::new_box::<EffHoles<'eval>>(
                    EffHoles::__call__(matches),
                    &__registration__::__EFF_HOLES_VTABLE,
                )),
            )
            .unwrap()
            .downcast_eval_ref(&__registration__::__EFF_HOLES_VTABLE)
    }
    pub(crate) fn max_hole_ilen<'eval>(
        &'eval self,
        __ctx: &dyn __EvalContext<'eval>,
    ) -> &'eval f32 {
        let __uid = entity_uid!(
            __ctx,
            "mnist_classifier::connected_component::ConnectedComponent::max_hole_ilen"
        );
        if let Some(__result) =
            __ctx.opt_cached_lazy_field(self as *const _ as *const std::ffi::c_void, __uid)
        {
            return __result
                .unwrap()
                .downcast_eval_ref(&__registration__::__F32_VTABLE);
        }
        let mut max_hole_ilen = 0;
        let raw_contours = &self.raw_contours(__ctx);
        for i in (0 + 1)..raw_contours.ilen() {
            let hole_ilen = raw_contours[(i) as usize].points.ilen();
            if max_hole_ilen < hole_ilen {
                max_hole_ilen = hole_ilen;
            }
        }
        __ctx
            .cache_lazy_field(
                self as *const _ as *const std::ffi::c_void,
                __uid,
                Ok(__Register::new_box::<f32>(
                    max_hole_ilen as f32,
                    &__registration__::__F32_VTABLE,
                )),
            )
            .unwrap()
            .downcast_eval_ref(&__registration__::__F32_VTABLE)
    }
    pub(crate) fn max_row_span<'eval>(&'eval self, __ctx: &dyn __EvalContext<'eval>) -> &'eval f32 {
        let __uid = entity_uid!(
            __ctx,
            "mnist_classifier::connected_component::ConnectedComponent::max_row_span"
        );
        if let Some(__result) =
            __ctx.opt_cached_lazy_field(self as *const _ as *const std::ffi::c_void, __uid)
        {
            return __result
                .unwrap()
                .downcast_eval_ref(&__registration__::__F32_VTABLE);
        }
        let mut max_row = 0;
        for i in (0 + 1)..29 {
            max_row = max_row.max(self.mask[(i) as usize].span());
        }
        __ctx
            .cache_lazy_field(
                self as *const _ as *const std::ffi::c_void,
                __uid,
                Ok(__Register::new_box::<f32>(
                    max_row as f32,
                    &__registration__::__F32_VTABLE,
                )),
            )
            .unwrap()
            .downcast_eval_ref(&__registration__::__F32_VTABLE)
    }
    pub(crate) fn row_span_sum<'eval>(&'eval self, __ctx: &dyn __EvalContext<'eval>) -> &'eval f32 {
        let __uid = entity_uid!(
            __ctx,
            "mnist_classifier::connected_component::ConnectedComponent::row_span_sum"
        );
        if let Some(__result) =
            __ctx.opt_cached_lazy_field(self as *const _ as *const std::ffi::c_void, __uid)
        {
            return __result
                .unwrap()
                .downcast_eval_ref(&__registration__::__F32_VTABLE);
        }
        let mut row_span_sum = 0;
        for i in (0 + 1)..29 {
            row_span_sum += self.mask[(i) as usize].span();
        }
        __ctx
            .cache_lazy_field(
                self as *const _ as *const std::ffi::c_void,
                __uid,
                Ok(__Register::new_box::<f32>(
                    row_span_sum as f32,
                    &__registration__::__F32_VTABLE,
                )),
            )
            .unwrap()
            .downcast_eval_ref(&__registration__::__F32_VTABLE)
    }
    pub(crate) fn distribution<'eval>(
        &'eval self,
        __ctx: &dyn __EvalContext<'eval>,
    ) -> &'eval ConnectedComponentDistribution {
        let __uid = entity_uid!(
            __ctx,
            "mnist_classifier::connected_component::ConnectedComponent::distribution"
        );
        if let Some(__result) =
            __ctx.opt_cached_lazy_field(self as *const _ as *const std::ffi::c_void, __uid)
        {
            return __result
                .unwrap()
                .downcast_eval_ref(&__registration__::__CONNECTED_COMPONENT_DISTRIBUTION_VTABLE);
        }
        let mut row_start = 1;
        while row_start < 29 {
            if self.mask[(row_start) as usize] != 0 {
                break;
            }
            row_start += 1;
        }
        let mut row_end = row_start + 1;
        while row_end < 29 {
            if (0 == self.mask[(row_end) as usize]) {
                break;
            }
            row_end += 1;
        }
        let height = row_end - row_start;
        let half_height = height / 2;
        let mut upper_mass = 0;
        for i1 in row_start..row_start + half_height {
            upper_mass += self.mask[(i1) as usize].co();
        }
        let mut lower_mass = 0;
        for i2 in row_end - half_height..row_end {
            lower_mass += self.mask[(i2) as usize].co();
        }
        __ctx
            .cache_lazy_field(
                self as *const _ as *const std::ffi::c_void,
                __uid,
                Ok(__Register::new_box::<ConnectedComponentDistribution>(
                    ConnectedComponentDistribution::__call__(
                        row_start, row_end, upper_mass, lower_mass,
                    ),
                    &__registration__::__CONNECTED_COMPONENT_DISTRIBUTION_VTABLE,
                )),
            )
            .unwrap()
            .downcast_eval_ref(&__registration__::__CONNECTED_COMPONENT_DISTRIBUTION_VTABLE)
    }
    pub(crate) fn upper_mass<'eval>(&'eval self, __ctx: &dyn __EvalContext<'eval>) -> &'eval f32 {
        let __uid = entity_uid!(
            __ctx,
            "mnist_classifier::connected_component::ConnectedComponent::upper_mass"
        );
        if let Some(__result) =
            __ctx.opt_cached_lazy_field(self as *const _ as *const std::ffi::c_void, __uid)
        {
            return __result
                .unwrap()
                .downcast_eval_ref(&__registration__::__F32_VTABLE);
        }
        return __ctx
            .cache_lazy_field(
                self as *const _ as *const std::ffi::c_void,
                __uid,
                Ok(__Register::new_box::<f32>(
                    self.distribution(__ctx).upper_mass as f32,
                    &__registration__::__F32_VTABLE,
                )),
            )
            .unwrap()
            .downcast_eval_ref(&__registration__::__F32_VTABLE);
    }
    pub(crate) fn lower_mass<'eval>(&'eval self, __ctx: &dyn __EvalContext<'eval>) -> &'eval f32 {
        let __uid = entity_uid!(
            __ctx,
            "mnist_classifier::connected_component::ConnectedComponent::lower_mass"
        );
        if let Some(__result) =
            __ctx.opt_cached_lazy_field(self as *const _ as *const std::ffi::c_void, __uid)
        {
            return __result
                .unwrap()
                .downcast_eval_ref(&__registration__::__F32_VTABLE);
        }
        return __ctx
            .cache_lazy_field(
                self as *const _ as *const std::ffi::c_void,
                __uid,
                Ok(__Register::new_box::<f32>(
                    self.distribution(__ctx).lower_mass as f32,
                    &__registration__::__F32_VTABLE,
                )),
            )
            .unwrap()
            .downcast_eval_ref(&__registration__::__F32_VTABLE);
    }
    pub(crate) fn top_k_row_span_sum(&self, k: i32) -> f32 {
        let mut top_k_row_span_sum = 0;
        assert!(k > 0);
        let mut i = 1;
        while i < 29 {
            if self.mask[(i) as usize] != 0 {
                break;
            }
            i += 1;
        }
        for j in i..i + k {
            top_k_row_span_sum += self.mask[(j) as usize].span();
        }
        return top_k_row_span_sum as f32;
    }

    pub(crate) fn top_k_row_right_mass_sum(&self, k: i32) -> f32 {
        let mut top_k_row_span_sum = 0;
        assert!(k > 0);
        let mut i = 1;
        while i < 29 {
            if self.mask[(i) as usize] != 0 {
                break;
            }
            i += 1;
        }
        for j in i..i + k {
            top_k_row_span_sum += self.mask[(j) as usize].right_mass();
        }
        return top_k_row_span_sum as f32;
    }
}

impl __StaticInfo for ConnectedComponent {
    type __StaticSelf = ConnectedComponent;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        "mnist_classifier::connected_component::ConnectedComponent".into()
    }

    unsafe fn __transmute_static(self) -> Self::__StaticSelf {
        std::mem::transmute(self)
    }
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
