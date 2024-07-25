use super::*;

#[allow(non_upper_case_globals)]
pub static mut __one_fermi_match__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::val(item_path_id_interface = __one_fermi_match__ITEM_PATH_ID_INTERFACE, return_leash)]
pub fn one_fermi_match() -> crate::fermi::FermiMatchResult {
    crate::fermi::fermi_match(major_concave_components(), &vec![downmost, upmost, hat])
}

#[allow(non_upper_case_globals)]
pub static mut __is_one__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::val(item_path_id_interface = __is_one__ITEM_PATH_ID_INTERFACE, lazy)]
pub fn is_one() -> malamute::OneVsAll {}

#[allow(non_upper_case_globals)]
pub static mut __upmost__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

#[rustfmt::skip]
pub fn upmost(cc: Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>) -> Option<f32> {
    let dp = cc.deleash().displacement();
    require!(dp.y > 0.0f32);
    Some(dp.y)
}

#[allow(non_upper_case_globals)]
pub static mut __downmost__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

#[rustfmt::skip]
pub fn downmost(cc: Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>) -> Option<f32> {
    let dp = cc.deleash().displacement();
    require!(dp.y <= 0.0f32);
    Some(-cc.deleash().end().y)
}

#[allow(non_upper_case_globals)]
pub static mut __hat__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

#[rustfmt::skip]
pub fn hat(cc: Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>) -> Option<f32> {
    let dp = cc.deleash().displacement();
    require!(dp.y < 0.0f32);
    require!(dp.x < 0.0f32);
    Some(-dp.y - dp.x)
}