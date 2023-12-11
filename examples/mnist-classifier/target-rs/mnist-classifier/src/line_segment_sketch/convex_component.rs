use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConvexComponent {
    pub line_segment_sketch: Leash<crate::line_segment_sketch::LineSegmentSketch>,
    pub line_segments: CyclicSliceLeashed<crate::line_segment_sketch::LineSegmentStroke>,
}

impl ConvexComponent {
    pub fn __constructor(line_segment_sketch: Leash<crate::line_segment_sketch::LineSegmentSketch>, line_segments: CyclicSliceLeashed<crate::line_segment_sketch::LineSegmentStroke>) -> Self {
        Self{
            line_segment_sketch,
            line_segments,
        }
    }
}

