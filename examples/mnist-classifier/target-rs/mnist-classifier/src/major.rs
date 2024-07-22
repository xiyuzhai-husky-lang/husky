use crate::*;

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::val(ingredient_index = 48, return_leash)]
pub fn connected_components() -> Vec<crate::connected_component::ConnectedComponent> {
    crate::connected_component::find_connected_components(INPUT())
}

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::val(ingredient_index = 49)]
pub fn major_connected_component() -> Leash<crate::connected_component::ConnectedComponent> {
    let mut i0 = 0;
    let mut max_row_span_sum = 0.0f32;
    for i in 0..connected_components().ilen() {
        let row_span_sum = <crate::connected_component::ConnectedComponent>::row_span_sum(connected_components()[Leash(&i) as usize]);
        if row_span_sum > max_row_span_sum {
            max_row_span_sum = row_span_sum;
            i0 = i
        }
    }
    return Leash(&connected_components()[i0 as usize]);
}

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::val(ingredient_index = 50)]
pub fn ignored_connected_components_row_span_sum_sum() -> f32 {
    let mut sum = 0.0f32;
    for i in 0..connected_components().ilen() {
        sum += <crate::connected_component::ConnectedComponent>::row_span_sum(connected_components()[Leash(&i) as usize])
    }
    return sum - <crate::connected_component::ConnectedComponent>::row_span_sum(major_connected_component().deleash());
}

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::val(ingredient_index = 51)]
pub fn major_raw_contours() -> Leash<Vec<crate::raw_contour::RawContour>> {
    Leash(&<crate::connected_component::ConnectedComponent>::raw_contours(major_connected_component().deleash()))
}

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::val(ingredient_index = 52)]
pub fn major_raw_contour() -> Leash<crate::raw_contour::RawContour> {
    Leash(&<crate::connected_component::ConnectedComponent>::raw_contours(major_connected_component().deleash())[0 as usize])
}

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::val(ingredient_index = 53)]
pub fn major_line_segment_sketch() -> Leash<crate::line_segment_sketch::LineSegmentSketch> {
    Leash(&<crate::raw_contour::RawContour>::line_segment_sketch(major_raw_contour().deleash()))
}

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::val(ingredient_index = 54)]
pub fn major_concave_components() -> Leash<Vec<crate::line_segment_sketch::concave_component::ConcaveComponent>> {
    Leash(&<crate::line_segment_sketch::LineSegmentSketch>::concave_components(major_line_segment_sketch().deleash()))
}