use super::*;

pub struct ConvexComponent {
    pub line_segment_sketch: Leash<LineSegmentSketch>,
    pub line_segments: Leash<CyclicSlice<LineSegmentStroke>>,
} 

impl Visualize for ConvexComponent {
    fn visualize(self) -> Html {
        self.line_segments.visualize()
    }
}