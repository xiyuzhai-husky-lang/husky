use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ConcaveComponent<'eval> {
    pub(crate) line_segment_sketch: &'eval crate::line_segment_sketch::LineSegmentSketch<'eval>,
    pub(crate) strokes:
        __std::slice::CyclicSlice<'eval, crate::line_segment_sketch::LineSegmentStroke<'eval>>,
}

impl<'eval> ConcaveComponent<'eval> {
    pub(crate) fn __call__(
        line_segment_sketch: &'eval crate::line_segment_sketch::LineSegmentSketch<'eval>,
        strokes: __std::slice::CyclicSlice<
            'eval,
            crate::line_segment_sketch::LineSegmentStroke<'eval>,
        >,
    ) -> Self {
        Self {
            line_segment_sketch,
            strokes,
        }
    }
    pub(crate) fn norm(&'eval self, __ctx: &dyn __EvalContext<'eval>) -> &'eval f32 {
        let __uid = entity_uid!(
            __ctx,
            "mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent::norm"
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
                Ok(__Register::new_eval_ref::<f32>(
                    &(self.hausdorff_norm(__ctx)),
                    &__registration__::__F32_VTABLE,
                ))
                .into(),
            )
            .unwrap()
            .downcast_eval_ref(&__registration__::__F32_VTABLE);
    }
    pub(crate) fn rel_norm(&'eval self, __ctx: &dyn __EvalContext<'eval>) -> &'eval f32 {
        let __uid = entity_uid!(
            __ctx,
            "mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent::rel_norm"
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
                    self.norm(__ctx) / self.displacement().norm(),
                    &__registration__::__F32_VTABLE,
                )),
            )
            .unwrap()
            .downcast_eval_ref(&__registration__::__F32_VTABLE);
    }
    pub(crate) fn hausdorff_norm(&'eval self, __ctx: &dyn __EvalContext<'eval>) -> &'eval f32 {
        let __uid = entity_uid!(__ctx, "mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent::hausdorff_norm");
        if let Some(__result) =
            __ctx.opt_cached_lazy_field(self as *const _ as *const std::ffi::c_void, __uid)
        {
            return __result
                .unwrap()
                .downcast_eval_ref(&__registration__::__F32_VTABLE);
        }
        let mut hausdorff_norm = 0f32;
        let curve_start = &self.strokes.firstx().start;
        let curve_ls = self.line_segment();
        let dp_norm = curve_ls.displacement().norm();
        for i in self.strokes.start..self.strokes.end {
            let point = &self.strokes[(i) as usize].end;
            let point_dist = curve_ls.dist_to_point(&point);
            if point_dist > hausdorff_norm {
                hausdorff_norm = point_dist;
            }
        }
        __ctx
            .cache_lazy_field(
                self as *const _ as *const std::ffi::c_void,
                __uid,
                Ok(__Register::new_box::<f32>(
                    hausdorff_norm,
                    &__registration__::__F32_VTABLE,
                )),
            )
            .unwrap()
            .downcast_eval_ref(&__registration__::__F32_VTABLE)
    }
    pub(crate) fn angle_change(&'eval self, __ctx: &dyn __EvalContext<'eval>) -> &'eval f32 {
        let __uid = entity_uid!(__ctx, "mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent::angle_change");
        if let Some(__result) =
            __ctx.opt_cached_lazy_field(self as *const _ as *const std::ffi::c_void, __uid)
        {
            return __result
                .unwrap()
                .downcast_eval_ref(&__registration__::__F32_VTABLE);
        }
        let mut angle_change = 0f32;
        let mut dp0 = self.strokes[(self.strokes.start) as usize].displacement();
        for i in (self.strokes.start + 1)..self.strokes.end {
            let dp = self.strokes[(i) as usize].displacement();
            angle_change += dp0.angle_to(&dp, true);
            dp0 = dp;
        }
        __ctx
            .cache_lazy_field(
                self as *const _ as *const std::ffi::c_void,
                __uid,
                Ok(__Register::new_box::<f32>(
                    angle_change,
                    &__registration__::__F32_VTABLE,
                )),
            )
            .unwrap()
            .downcast_eval_ref(&__registration__::__F32_VTABLE)
    }
    pub(crate) fn bounding_box(
        &'eval self,
        __ctx: &dyn __EvalContext<'eval>,
    ) -> &'eval crate::geom2d::BoundingBox {
        let __uid = entity_uid!(__ctx, "mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent::bounding_box");
        if let Some(__result) =
            __ctx.opt_cached_lazy_field(self as *const _ as *const std::ffi::c_void, __uid)
        {
            return __result
                .unwrap()
                .downcast_eval_ref(&__registration__::__BOUNDING_BOX_VTABLE);
        }
        let start_point = &self.strokes.firstx().start;
        let mut xmin = start_point.x;
        let mut xmax = start_point.x;
        let mut ymin = start_point.y;
        let mut ymax = start_point.y;
        for i in self.strokes.start..self.strokes.end {
            let point = &self.strokes[(i) as usize].end;
            xmin = xmin.min(point.x);
            xmax = xmax.max(point.x);
            ymin = ymin.min(point.y);
            ymax = ymax.max(point.y);
        }
        __ctx
            .cache_lazy_field(
                self as *const _ as *const std::ffi::c_void,
                __uid,
                Ok(__Register::new_box::<crate::geom2d::BoundingBox>(
                    crate::geom2d::BoundingBox::__call__(
                        crate::geom2d::ClosedRange::__call__(xmin, xmax),
                        crate::geom2d::ClosedRange::__call__(ymin, ymax),
                    ),
                    &__registration__::__BOUNDING_BOX_VTABLE,
                )),
            )
            .unwrap()
            .downcast_eval_ref(&__registration__::__BOUNDING_BOX_VTABLE)
    }
    pub(crate) fn relative_bounding_box(
        &'eval self,
        __ctx: &dyn __EvalContext<'eval>,
    ) -> &'eval crate::geom2d::RelativeBoundingBox {
        let __uid = entity_uid!(__ctx, "mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent::relative_bounding_box");
        if let Some(__result) =
            __ctx.opt_cached_lazy_field(self as *const _ as *const std::ffi::c_void, __uid)
        {
            return __result
                .unwrap()
                .downcast_eval_ref(&__registration__::__RELATIVE_BOUNDING_BOX_VTABLE);
        }
        return __ctx
            .cache_lazy_field(
                self as *const _ as *const std::ffi::c_void,
                __uid,
                Ok(__Register::new_box::<crate::geom2d::RelativeBoundingBox>(
                    self.line_segment_sketch
                        .bounding_box(__ctx)
                        .relative_bounding_box(&self.bounding_box(__ctx)),
                    &__registration__::__RELATIVE_BOUNDING_BOX_VTABLE,
                )),
            )
            .unwrap()
            .downcast_eval_ref(&__registration__::__RELATIVE_BOUNDING_BOX_VTABLE);
    }
    pub(crate) fn line_segment(&self) -> crate::line_segment_sketch::line_segment::LineSegment {
        return crate::line_segment_sketch::line_segment::LineSegment::__call__(
            self.strokes.firstx().start.clone(),
            self.strokes.lastx().end.clone(),
        );
    }

    pub(crate) fn start(&self) -> crate::geom2d::Point2d {
        return self.strokes.firstx().start.clone();
    }

    pub(crate) fn end(&self) -> crate::geom2d::Point2d {
        return self.strokes.lastx().end.clone();
    }

    pub(crate) fn displacement(&self) -> crate::geom2d::Vector2d {
        return self.line_segment().displacement();
    }

    pub(crate) fn start_tangent(&self) -> crate::geom2d::Vector2d {
        return self.strokes.firstx().displacement();
    }

    pub(crate) fn end_tangent(&self) -> crate::geom2d::Vector2d {
        return self.strokes.lastx().displacement();
    }
}

impl<'eval> __StaticInfo for ConcaveComponent<'eval> {
    type __StaticSelf = ConcaveComponent<'static>;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        "mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent".into()
    }

    unsafe fn __transmute_static(self) -> Self::__StaticSelf {
        std::mem::transmute(self)
    }
}

pub(crate) fn find_concave_components<'eval>(
    line_segment_sketch: &'eval crate::line_segment_sketch::LineSegmentSketch<'eval>,
) -> Vec<ConcaveComponent<'eval>> {
    let mut concave_components = Vec::<ConcaveComponent>::__call__(vec![]);
    let L = line_segment_sketch.strokes.ilen();
    let mut start = 0;
    let mut end = 1;
    while start > -L
        && !crate::line_segment_sketch::convexity::is_convex(&line_segment_sketch, start)
    {
        start -= 1;
    }
    let ccv_start = start;
    while start < ccv_start + L {
        while end <= start + L
            && !crate::line_segment_sketch::convexity::is_convex(&line_segment_sketch, end)
        {
            end += 1;
        }
        if end > start + 1 {
            concave_components.push(ConcaveComponent::__call__(
                &line_segment_sketch,
                line_segment_sketch.strokes.cyclic_slice(start, end),
            ));
        }
        start = end;
        end = start + 1;
    }
    return concave_components;
}
