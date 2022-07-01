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
