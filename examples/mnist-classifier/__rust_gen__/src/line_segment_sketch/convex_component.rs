use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ConvexCompoent<'eval> {
    pub(crate) line_segment_sketch: &'eval crate::line_segment_sketch::LineSegmentSketch<'eval>,
    pub(crate) line_segments:
        __std::slice::CyclicSlice<'eval, crate::line_segment_sketch::LineSegmentStroke<'eval>>,
}

impl<'eval> ConvexCompoent<'eval> {
    pub(crate) fn __call__(
        line_segment_sketch: &'eval crate::line_segment_sketch::LineSegmentSketch<'eval>,
        line_segments: __std::slice::CyclicSlice<
            'eval,
            crate::line_segment_sketch::LineSegmentStroke<'eval>,
        >,
    ) -> Self {
        Self {
            line_segment_sketch,
            line_segments,
        }
    }
}

impl<'eval> __StaticInfo for ConvexCompoent<'eval> {
    type __StaticSelf = ConvexCompoent<'static>;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        "mnist_classifier::line_segment_sketch::convex_component::ConvexCompoent".into()
    }

    unsafe fn __transmute_static(self) -> Self::__StaticSelf {
        std::mem::transmute(self)
    }
}
