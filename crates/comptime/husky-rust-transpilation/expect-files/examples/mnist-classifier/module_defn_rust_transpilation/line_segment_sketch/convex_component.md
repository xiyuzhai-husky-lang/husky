struct!ConvexComponent{line_segment_sketch:Leash<LineSegmentSketch>, line_segments:Leash<CyclicSlice<LineSegmentStroke>>}impl!fn!visualize(){
    Self.line_segments.visualize();
}
