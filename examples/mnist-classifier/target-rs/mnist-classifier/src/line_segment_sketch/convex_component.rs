struct ConvexComponent{
    line_segment_sketch: Leash<LineSegmentSketch>,
    line_segments: Leash<CyclicSlice<LineSegmentStroke>>,
}

impl Visualizefor ConvexComponent {
    fn visualize(self) {
        self.line_segments.visualize()
    }
}