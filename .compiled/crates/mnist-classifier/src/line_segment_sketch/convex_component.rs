use crate::*;

pub struct ConvexCompoent<'eval> {
    pub(crate) line_segment_sketch: &'eval crate::line_segment_sketch::LineSegmentSketch<'eval>,
    pub(crate) line_segments: __std::slice::CyclicSlice<'eval, crate::line_segment_sketch::LineSegment<'eval>>,
}

impl<'eval> ConvexCompoent<'eval> {
}
