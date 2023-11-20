use super::*;

pub struct ConvexComponent{
    line_segment_sketch: Leash<LineSegmentSketch>,
    line_segments: Leash<CyclicSlice<LineSegmentStroke>>,
}

impl Visualize for ConvexComponent {
    fn visualize(self) -> Html {
        self.line_segments.visualize()
    }
}