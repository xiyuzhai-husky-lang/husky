use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ConvexCompoent<'eval> {
    pub(crate) line_segment_sketch: &'eval crate::line_segment_sketch::LineSegmentSketch<'eval>,
    pub(crate) line_segments: __std::slice::CyclicSlice<'eval, crate::line_segment_sketch::LineSegment<'eval>>,
}

impl<'eval> ConvexCompoent<'eval> {
    pub(crate) fn __call__(line_segment_sketch: &'eval crate::line_segment_sketch::LineSegmentSketch<'eval>, line_segments: __std::slice::CyclicSlice<'eval, crate::line_segment_sketch::LineSegment<'eval>>) -> Self {
        Self { line_segment_sketch, line_segments }
    }
}

impl<'eval> __HasStaticTypeInfo for ConvexCompoent<'eval> {
    type StaticSelf = ConvexCompoent<'static>;

    fn static_type_name() -> std::borrow::Cow<'static, str> {
        todo!()
    }
}

impl<'eval> __AnyValue<'eval> for ConvexCompoent<'eval> {
    fn print_short(&self) -> String {
        todo!()
    }

    fn to_json_value(&self) -> __JsonValue {
        todo!()
    }

    fn short<'short>(&self) -> &dyn __AnyValueDyn<'short>
    where
        'eval: 'short {
        todo!()
    }

    fn ty(&self) -> __EntityRoutePtr {
        todo!()
    }
}
