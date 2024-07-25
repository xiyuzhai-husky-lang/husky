use super::*;

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::val(item_path_id_interface = 24, return_leash)]
pub fn open_one_match() -> crate::fermi::FermiMatchResult {
    crate::fermi::fermi_match(major_concave_components(), &vec![almost_closed])
}

#[rustfmt::skip]
pub fn almost_closed(cc: Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>) -> Option<f32> {
    require!(<crate::line_segment_sketch::concave_component::ConcaveComponent>::angle_change(cc) + 0.0f32 < -140.0f32);
    Some(-<crate::line_segment_sketch::concave_component::ConcaveComponent>::angle_change(cc) + 0.0f32)
}

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::val(item_path_id_interface = 25, lazy)]
pub fn is_zero() -> malamute::OneVsAll {}