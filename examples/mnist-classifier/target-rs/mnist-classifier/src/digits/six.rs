use super::*;

pub static mut __six_match__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::val(item_path_id_interface = __six_match__ITEM_PATH_ID_INTERFACE, return_leash)]
pub fn six_match() -> crate::fermi::FermiMatchResult {
    crate::fermi::fermi_match(major_concave_components(), &vec![upmost])
}

pub static mut __six_match_refined1__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::val(item_path_id_interface = __six_match_refined1__ITEM_PATH_ID_INTERFACE, return_leash)]
pub fn six_match_refined1() -> crate::fermi::FermiMatchResult {
    crate::fermi::fermi_match(major_concave_components(), &vec![upmost, bottom1])
}

pub static mut __is_six__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::val(item_path_id_interface = __is_six__ITEM_PATH_ID_INTERFACE, lazy)]
pub fn is_six() -> malamute::OneVsAll {}

pub static mut __upmost__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

#[rustfmt::skip]
pub fn upmost(cc: Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>) -> Option<f32> {
    let dp = cc.deleash().displacement();
    require!(dp.y > 0.0f32);
    Some(dp.y)
}

pub static mut __bottom1__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

#[rustfmt::skip]
pub fn bottom1(cc: Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>) -> Option<f32> {
    let dp = cc.deleash().displacement();
    if dp.y < -3.0f32 {
        require!((dp.x / dp.y).abs() > 1.4f32);
    }
    require!(<crate::line_segment_sketch::concave_component::ConcaveComponent>::relative_bounding_box(cc).deleash().ymax() < 0.6f32);
    let relative_end = <crate::line_segment_sketch::LineSegmentSketch>::bounding_box(cc.deleash().line_segment_sketch).deleash().relative_point(&cc.deleash().end());
    require!(relative_end.x > 0.5f32);
    Some(-cc.deleash().end().y)
}