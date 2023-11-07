
fn connected_components() {
    find_connected_components(input)
}

fn major_connected_component() {
    let mut i0 = 0;
    let mut max_row_span_sum = 0;
    for i in 0..connected_components.ilen() {
        let row_span_sum = connected_components[v2].row_span_sum;
        if v3 > v1 {
            v1 = v3;
            v0 = v2
        }
    }
    return connected_components[v0];
}

fn ignored_connected_components_row_span_sum_sum() {
    let mut sum = 0;
    for i in 0..connected_components.ilen() {
        v0 += connected_components[v1].row_span_sum
    }
    return v0 - major_connected_component.row_span_sum;
}

fn major_raw_contours() {
    major_connected_component.raw_contours
}

fn major_raw_contour() {
    major_connected_component.raw_contours[0]
}

fn major_line_segment_sketch() {
    major_raw_contour.line_segment_sketch
}

fn major_concave_components() {
    major_line_segment_sketch.concave_components
}