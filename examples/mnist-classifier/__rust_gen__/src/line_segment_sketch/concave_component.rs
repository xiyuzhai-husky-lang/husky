use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ConcaveComponent<'eval> {
    pub(crate) line_segment_sketch: &'eval crate::line_segment_sketch::LineSegmentSketch<'eval>,
    pub(crate) line_segments:
        __std::slice::CyclicSlice<'eval, crate::line_segment_sketch::LineSegment<'eval>>,
}

impl<'eval> ConcaveComponent<'eval> {
    pub(crate) fn __call__(
        line_segment_sketch: &'eval crate::line_segment_sketch::LineSegmentSketch<'eval>,
        line_segments: __std::slice::CyclicSlice<
            'eval,
            crate::line_segment_sketch::LineSegment<'eval>,
        >,
    ) -> Self {
        Self {
            line_segment_sketch,
            line_segments,
        }
    }
    pub(crate) fn norm(&'eval self, __ctx: &dyn __EvalContext<'eval>) -> &'eval f32 {
        let __uid = entity_uid!(
            __ctx,
            "mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent::norm"
        );
        if let Some(__result) = __ctx.opt_cached_lazy_field(self as *const _ as *const (), __uid) {
            return __result
                .unwrap()
                .downcast_eval_ref(&__registration__::__F32_VTABLE);
        }
        return __ctx
            .cache_lazy_field(
                self as *const _ as *const (),
                __uid,
                Ok(__Register::new_box::<f32>(
                    1f32,
                    &__registration__::__F32_VTABLE,
                )),
            )
            .unwrap()
            .downcast_eval_ref(&__registration__::__F32_VTABLE);
    }
    pub(crate) fn displacement(
        &'eval self,
        __ctx: &dyn __EvalContext<'eval>,
    ) -> &'eval crate::geom2d::Vector2d {
        let __uid = entity_uid!(__ctx, "mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent::displacement");
        if let Some(__result) = __ctx.opt_cached_lazy_field(self as *const _ as *const (), __uid) {
            return __result
                .unwrap()
                .downcast_eval_ref(&__registration__::__VECTOR_2_D_VTABLE);
        }
        return __ctx
            .cache_lazy_field(
                self as *const _ as *const (),
                __uid,
                Ok(__Register::new_box::<crate::geom2d::Vector2d>(
                    self.line_segments
                        .firstx()
                        .start
                        .to(&self.line_segments.lastx().end),
                    &__registration__::__VECTOR_2_D_VTABLE,
                )),
            )
            .unwrap()
            .downcast_eval_ref(&__registration__::__VECTOR_2_D_VTABLE);
    }
}

impl<'eval> __StaticInfo for ConcaveComponent<'eval> {
    type __StaticSelf = ConcaveComponent<'static>;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        "mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent".into()
    }
}

impl<'eval> __Registrable<'eval> for ConcaveComponent<'eval> {
    unsafe fn __to_register(self) -> __Register<'eval> {
        todo!()
    }
}

pub(crate) fn find_concave_components<'eval>(
    line_segment_sketch: &'eval crate::line_segment_sketch::LineSegmentSketch<'eval>,
) -> Vec<ConcaveComponent<'eval>> {
    let mut concave_components = Vec::<ConcaveComponent>::__call__(vec![]);
    let L = line_segment_sketch.line_segments.ilen();
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
                line_segment_sketch,
                line_segment_sketch.line_segments.cyclic_slice(start, end),
            ));
        }
        start = end;
        end = start + 1;
    }
    return concave_components;
}
