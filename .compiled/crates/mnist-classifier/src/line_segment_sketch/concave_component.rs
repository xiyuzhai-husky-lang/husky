use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ConcaveComponent<'eval> {
    pub(crate) line_segment_sketch: &'eval crate::line_segment_sketch::LineSegmentSketch<'eval>,
    pub(crate) line_segments: __std::slice::CyclicSlice<'eval, crate::line_segment_sketch::LineSegment<'eval>>,
}

impl<'eval> ConcaveComponent<'eval> {
    pub(crate) fn __call__(line_segment_sketch: &'eval crate::line_segment_sketch::LineSegmentSketch<'eval>, line_segments: __std::slice::CyclicSlice<'eval, crate::line_segment_sketch::LineSegment<'eval>>) -> Self {
        Self { line_segment_sketch, line_segments }
    }
}

impl<'eval> __HasStaticTypeInfo for ConcaveComponent<'eval> {
    type StaticSelf = ConcaveComponent<'static>;

    fn static_type_name() -> std::borrow::Cow<'static, str> {
        "mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent".into()
    }
}

impl<'eval> __AnyValue<'eval> for ConcaveComponent<'eval> {
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

    fn static_ty() -> __EntityRoutePtr {
        __lazy_entity_route_from_text!("mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent")
    }
}

pub(crate) fn find_concave_components<'eval>(line_segment_sketch: &'eval crate::line_segment_sketch::LineSegmentSketch<'eval>) -> Vec<ConcaveComponent<'eval>> {
    let mut concave_components = Vec::<ConcaveComponent>::__call__();
    let L = line_segment_sketch.line_segments.ilen();
    let mut start = 0;
    let mut end = 1;
    while start > -L && !crate::line_segment_sketch::convexity::is_convex(&line_segment_sketch, start) {
        start -= 1;
    }
    let ccv_start = start;
    while start < ccv_start + L {
        while end <= start + L && !crate::line_segment_sketch::convexity::is_convex(&line_segment_sketch, end) {
            end += 1;
        }
        if end > start + 1 {
            concave_components.push(ConcaveComponent::__call__(line_segment_sketch, line_segment_sketch.line_segments.cyclic_slice(start, end)));
        }
        start = end;
        end = start + 1;
    }
    return concave_components
}
