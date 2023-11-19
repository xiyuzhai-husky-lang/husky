
struct ConvexComponent{
    line_segment_sketch: Leash<LineSegmentSketch>,
    line_segments: Leash<CyclicSlice<LineSegmentStroke>>,
}