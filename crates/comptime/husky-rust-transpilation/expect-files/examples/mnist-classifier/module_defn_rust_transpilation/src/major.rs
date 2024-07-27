use crate::*;

#[allow(non_upper_case_globals)]
pub static mut __connected_components__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::val(
    item_path_id_interface = __connected_components__ITEM_PATH_ID_INTERFACE,
    var_deps = [mnist::INPUT],
    return_leash
)]
pub fn connected_components() -> Vec<crate::connected_component::ConnectedComponent> {
    crate::connected_component::find_connected_components(INPUT().deleash())
}

#[allow(non_upper_case_globals)]
pub static mut __major_connected_component__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::val(
    item_path_id_interface = __major_connected_component__ITEM_PATH_ID_INTERFACE,
    var_deps = [mnist::INPUT]
)]
pub fn major_connected_component() -> Leash<crate::connected_component::ConnectedComponent> {
    let mut i0 = 0;
    let mut max_row_span_sum = 0.0f32;
    for i in 0..connected_components().deleash().ilen() {
        let row_span_sum = <crate::connected_component::ConnectedComponent>::row_span_sum(Leash(&connected_components().deleash()[i as usize]));
        if row_span_sum > max_row_span_sum {
            max_row_span_sum = row_span_sum;
            i0 = i
        }
    }
    return Leash(&connected_components().deleash()[i0 as usize]);
}

#[allow(non_upper_case_globals)]
pub static mut __ignored_connected_components_row_span_sum_sum__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::val(
    item_path_id_interface = __ignored_connected_components_row_span_sum_sum__ITEM_PATH_ID_INTERFACE,
    var_deps = [mnist::INPUT]
)]
pub fn ignored_connected_components_row_span_sum_sum() -> f32 {
    let mut sum = 0.0f32;
    for i in 0..connected_components().deleash().ilen() {
        sum += <crate::connected_component::ConnectedComponent>::row_span_sum(Leash(&connected_components().deleash()[i as usize]))
    }
    return sum - <crate::connected_component::ConnectedComponent>::row_span_sum(major_connected_component());
}

#[allow(non_upper_case_globals)]
pub static mut __major_raw_contours__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::val(
    item_path_id_interface = __major_raw_contours__ITEM_PATH_ID_INTERFACE,
    var_deps = [mnist::INPUT]
)]
pub fn major_raw_contours() -> Leash<Vec<crate::raw_contour::RawContour>> {
    <crate::connected_component::ConnectedComponent>::raw_contours(major_connected_component())
}

#[allow(non_upper_case_globals)]
pub static mut __major_raw_contour__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::val(
    item_path_id_interface = __major_raw_contour__ITEM_PATH_ID_INTERFACE,
    var_deps = [mnist::INPUT]
)]
pub fn major_raw_contour() -> Leash<crate::raw_contour::RawContour> {
    Leash(&<crate::connected_component::ConnectedComponent>::raw_contours(major_connected_component()).deleash()[0 as usize])
}

#[allow(non_upper_case_globals)]
pub static mut __major_line_segment_sketch__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::val(
    item_path_id_interface = __major_line_segment_sketch__ITEM_PATH_ID_INTERFACE,
    var_deps = [mnist::INPUT]
)]
pub fn major_line_segment_sketch() -> Leash<crate::line_segment_sketch::LineSegmentSketch> {
    <crate::raw_contour::RawContour>::line_segment_sketch(major_raw_contour())
}

#[allow(non_upper_case_globals)]
pub static mut __major_concave_components__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::val(
    item_path_id_interface = __major_concave_components__ITEM_PATH_ID_INTERFACE,
    var_deps = [mnist::INPUT]
)]
pub fn major_concave_components() -> Leash<Vec<crate::line_segment_sketch::concave_component::ConcaveComponent>> {
    <crate::line_segment_sketch::LineSegmentSketch>::concave_components(major_line_segment_sketch())
}