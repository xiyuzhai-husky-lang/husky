use super::*;

pub struct ConvexComponent {
    pub line_segment_sketch: Leash<LineSegmentSketch>,
    pub line_segments: Leash<CyclicSlice<LineSegmentStroke>>,
}
