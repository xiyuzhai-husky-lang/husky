
struct ConcaveComponent{line_segment_sketch: Leash<LineSegmentSketch>, strokes: Leash<CyclicSlice<LineSegmentStroke>>}

pub fn find_concave_components(line_segment_sketch: Leash<LineSegmentSketch>) -> Vec<ConcaveComponent> {
    let mut concave_components: Vec<ConcaveComponent> = vec![];
    let L = v0.strokes.ilen();
    let mut start = 0;
    let mut end = 1;
    while v3 > -v2 && !is_convex(v0, v3) {
        v3-= 1
    }
    let ccv_start = v3;
    while v3 < v5 + v2 {
        while v4 <= v3 + v2 && !is_convex(v0, v4) {
            v4+= 1
        }
        if v4 > v3 + 1 {
            v1.push(ConcaveComponent(v0, v0.strokes.cyclic_slice_leashed(v3, v4)))
        }
        v3 = v4;
        v4 = v3 + 1
    }
    return v1;
}