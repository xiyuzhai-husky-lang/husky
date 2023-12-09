use crate::*;

#[ad_hoc_task_dependency::val_item_return_ref(48)]
pub fn connected_components() -> Vec<ConnectedComponent> {
    find_connected_components(input())
}

#[ad_hoc_task_dependency::val_item(49)]
pub fn major_connected_component() -> Leash<ConnectedComponent> {
    let mut i0 = 0;
    let mut max_row_span_sum = 0.0f32;
    for i in 0..connected_components().ilen() {
        let row_span_sum = connected_components()[i as usize].row_span_sum();
        if row_span_sum > max_row_span_sum {
            max_row_span_sum = row_span_sum;
            i0 = i
        }
    }
    return &connected_components()[i0 as usize];
}

#[ad_hoc_task_dependency::val_item(50)]
pub fn ignored_connected_components_row_span_sum_sum() -> f32 {
    let mut sum = 0.0f32;
    for i in 0..connected_components().ilen() {
        sum += connected_components()[i as usize].row_span_sum()
    }
    return sum - major_connected_component().row_span_sum();
}

#[ad_hoc_task_dependency::val_item(51)]
pub fn major_raw_contours() -> Leash<Vec<RawContour>> {
    &major_connected_component().raw_contours()
}

#[ad_hoc_task_dependency::val_item(52)]
pub fn major_raw_contour() -> Leash<RawContour> {
    &major_connected_component().raw_contours()[0 as usize]
}

#[ad_hoc_task_dependency::val_item(53)]
pub fn major_line_segment_sketch() -> Leash<LineSegmentSketch> {
    &major_raw_contour().line_segment_sketch()
}

#[ad_hoc_task_dependency::val_item(54)]
pub fn major_concave_components() -> Leash<Vec<ConcaveComponent>> {
    &major_line_segment_sketch().concave_components()
}