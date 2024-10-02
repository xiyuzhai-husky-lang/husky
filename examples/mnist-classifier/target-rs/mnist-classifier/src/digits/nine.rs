use super::*;

#[allow(non_upper_case_globals)]
pub static mut __nine_match__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::val(
    item_path_id_interface = __nine_match__ITEM_PATH_ID_INTERFACE,
    var_deps = [mnist::INPUT],
    return_leash
)]
pub fn nine_match() -> crate::fermi::FermiMatchResult {
    crate::fermi::fermi_match(major_concave_components(), &vec![downmost])
}

#[allow(non_upper_case_globals)]
pub static mut __nine_match_refine__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::val(
    item_path_id_interface = __nine_match_refine__ITEM_PATH_ID_INTERFACE,
    var_deps = [mnist::INPUT],
    return_leash
)]
pub fn nine_match_refine() -> crate::fermi::FermiMatchResult {
    crate::fermi::fermi_match(major_concave_components(), &vec![big_cc])
}

#[allow(non_upper_case_globals)]
pub static mut __is_nine__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::val(
    item_path_id_interface = __is_nine__ITEM_PATH_ID_INTERFACE,
    var_deps = [mnist::INPUT]
)]
pub fn is_nine() -> malamute::OneVsAll {
    let eff_holes = <crate::connected_component::ConnectedComponent>::eff_holes(major_connected_component());
    require!(let Option::None = *eff_holes.deleash().matches.index(1 as usize));
    let down_match = *nine_match().deleash().matches.index(0 as usize);
    require!(let Some(_) = down_match);
    let down_match_dp_y = down_match.unwrap().deleash().displacement().y;
    let higher_excess = <crate::connected_component::ConnectedComponent>::upper_mass(major_connected_component()) - <crate::connected_component::ConnectedComponent>::lower_mass(major_connected_component());
    require!(higher_excess > 7.0f32);
    if let Option::None = *eff_holes.deleash().matches.index(0 as usize) {
        require!(major_concave_components().deleash().ilen() >= 2);
        let nine_match_refine_result = *nine_match_refine().deleash().matches.index(0 as usize);
        require!(let Some(_) = nine_match_refine_result);
        require!(<crate::fermi::FermiMatchResult>::norm(nine_match_refine()) < 1.0f32);
        let higher_excess = <crate::connected_component::ConnectedComponent>::upper_mass(major_connected_component()) - <crate::connected_component::ConnectedComponent>::lower_mass(major_connected_component());
        let upper_arc = *nine_match_refine().deleash().matches.index(0 as usize);
        require!(let Some(_) = upper_arc);
        require!(upper_arc.unwrap().deleash().displacement().y > 0.0f32);
        require!(<crate::line_segment_sketch::concave_component::ConcaveComponent>::angle_change(upper_arc.unwrap()) < -110.0f32);
        require!(<crate::fermi::FermiMatchResult>::norm(nine_match_refine()) < 9.0f32);
        let a = major_connected_component().deleash().top_k_row_right_mass_sum(3);
        require!(a < 22.0f32);
        require!(a > 9.0f32);
        return OneVsAll::Yes;
    }
    OneVsAll::Yes
}

#[allow(non_upper_case_globals)]
pub static mut __downmost__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

#[rustfmt::skip]
pub fn downmost(cc: Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>) -> Option<f32> {
    let dp = cc.deleash().displacement();
    require!(dp.y < 0.0f32);
    Some(dp.y)
}

#[allow(non_upper_case_globals)]
pub static mut __big_cc__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

#[rustfmt::skip]
pub fn big_cc(cc: Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>) -> Option<f32> {
    let dp = cc.deleash().displacement();
    require!(dp.y > 0.0f32);
    require!(<crate::line_segment_sketch::concave_component::ConcaveComponent>::relative_bounding_box(cc).deleash().ymin() > 0.4f32);
    Some(<crate::line_segment_sketch::concave_component::ConcaveComponent>::relative_bounding_box(cc).deleash().ymin())
}