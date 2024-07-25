use super::*;

#[allow(non_upper_case_globals)]
pub static mut __open_one_match__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::val(item_path_id_interface = __open_one_match__ITEM_PATH_ID_INTERFACE, return_leash)]
pub fn open_one_match() -> crate::fermi::FermiMatchResult {
    crate::fermi::fermi_match(major_concave_components(), &vec![almost_closed])
}

#[allow(non_upper_case_globals)]
pub static mut __almost_closed__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

#[rustfmt::skip]
pub fn almost_closed(cc: Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>) -> Option<f32> {
    require!(<crate::line_segment_sketch::concave_component::ConcaveComponent>::angle_change(cc) + 0.0f32 < -140.0f32);
    Some(-<crate::line_segment_sketch::concave_component::ConcaveComponent>::angle_change(cc) + 0.0f32)
}

#[allow(non_upper_case_globals)]
pub static mut __is_zero__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::val(item_path_id_interface = __is_zero__ITEM_PATH_ID_INTERFACE, lazy)]
pub fn is_zero() -> malamute::OneVsAll {}