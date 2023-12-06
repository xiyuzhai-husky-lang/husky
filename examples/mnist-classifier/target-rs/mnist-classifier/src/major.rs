use crate::*;

pub fn connected_components() -> Vec<ConnectedComponent> {
    find_connected_components(input())
}

pub fn major_connected_component() -> Leash<ConnectedComponent> {
    let mut i0 = 0;
    let mut max_row_span_sum = 0;
    for i in 0..connected_components().ilen() {
        let row_span_sum = connected_components()[i].row_span_sum();
        if row_span_sum > max_row_span_sum {
            max_row_span_sum = row_span_sum;
            i0 = i
        }
    }
    return connected_components()[i0];
}

pub fn ignored_connected_components_row_span_sum_sum() -> f32 {
    let mut sum = 0;
    for i in 0..connected_components().ilen() {
        sum += connected_components()[i].row_span_sum()
    }
    return sum - major_connected_component().row_span_sum();
}

pub fn major_raw_contours() -> Leash<Vec<RawContour>> {
    &major_connected_component().raw_contours()
}

pub fn major_raw_contour() -> Leash<RawContour> {
    &major_connected_component().raw_contours()[0]
}

pub fn major_line_segment_sketch() -> Leash<LineSegmentSketch> {
    &major_raw_contour().line_segment_sketch()
}

pub fn major_concave_components() -> Leash<Vec<ConcaveComponent>> {
    &major_line_segment_sketch().concave_components()
}