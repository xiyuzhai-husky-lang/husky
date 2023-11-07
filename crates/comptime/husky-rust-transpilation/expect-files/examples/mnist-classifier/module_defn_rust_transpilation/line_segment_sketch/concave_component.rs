
struct ConcaveComponent{line_segment_sketch: Leash<LineSegmentSketch>, strokes: Leash<CyclicSlice<LineSegmentStroke>>}

pub fn find_concave_components(line_segment_sketch: Leash<LineSegmentSketch>) -> Vec<ConcaveComponent> {
    let mut concave_components: Vec<ConcaveComponent> = vec![];
    let L = line_segment_sketch.strokes.ilen();
    let mut start = 0;
    let mut end = 1;
    while start > -L && !is_convex(line_segment_sketch, start) {
        start-= 1
    }
    let ccv_start = start;
    while start < ccv_start + L {
        while end <= start + L && !is_convex(line_segment_sketch, end) {
            end+= 1
        }
        if end > start + 1 {
            concave_components.push(ConcaveComponent(line_segment_sketch, line_segment_sketch.strokes.cyclic_slice_leashed(start, end)))
        }
        start = end;
        end = start + 1
    }
    return concave_components;
}