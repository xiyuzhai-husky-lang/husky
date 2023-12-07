use super::*;

pub struct ConvexComponent {
    pub line_segment_sketch: Leash<LineSegmentSketch>,
    pub line_segments: Leash<CyclicSlice<LineSegmentStroke>>,
} 

impl ConvexComponent {
    pub fn __constructor(line_segment_sketch: Leash<LineSegmentSketch>, line_segments: Leash<CyclicSlice<LineSegmentStroke>>) -> Self {
        Self{
            line_segment_sketch,
            line_segments,
        }
    }
}

