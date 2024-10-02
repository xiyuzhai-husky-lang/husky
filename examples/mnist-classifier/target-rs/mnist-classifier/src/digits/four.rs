use super::*;

#[allow(non_upper_case_globals)]
pub static mut __left_components__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::val(
    item_path_id_interface = __left_components__ITEM_PATH_ID_INTERFACE,
    var_deps = [mnist::INPUT],
    return_leash
)]
pub fn left_components() -> crate::fermi::FermiMatchResult {
    crate::fermi::fermi_match(major_concave_components(), &vec![left_coordinate_max, left_coordinate_max])
}

#[allow(non_upper_case_globals)]
pub static mut __left_coordinate_max__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

#[rustfmt::skip]
pub fn left_coordinate_max(cc: Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>) -> Option<f32> {
    Some(<crate::line_segment_sketch::concave_component::ConcaveComponent>::relative_bounding_box(cc).deleash().xmax())
}

#[allow(non_upper_case_globals)]
pub static mut __components_max_downwards__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::val(
    item_path_id_interface = __components_max_downwards__ITEM_PATH_ID_INTERFACE,
    var_deps = [mnist::INPUT],
    return_leash
)]
pub fn components_max_downwards() -> crate::fermi::FermiMatchResult {
    crate::fermi::fermi_match(major_concave_components(), &vec![displacement_downwards])
}

#[allow(non_upper_case_globals)]
pub static mut __components_max_heights__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::val(
    item_path_id_interface = __components_max_heights__ITEM_PATH_ID_INTERFACE,
    var_deps = [mnist::INPUT],
    return_leash
)]
pub fn components_max_heights() -> crate::fermi::FermiMatchResult {
    crate::fermi::fermi_match(major_concave_components(), &vec![cc_box_heights])
}

#[allow(non_upper_case_globals)]
pub static mut __is_four__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::val(
    item_path_id_interface = __is_four__ITEM_PATH_ID_INTERFACE,
    var_deps = [mnist::INPUT]
)]
pub fn is_four() -> malamute::OneVsAll {
    require!(let Some(_) = *left_components().deleash().matches.index(0 as usize));
    require!(let Some(_) = *left_components().deleash().matches.index(1 as usize));
    let eff_holes = <crate::connected_component::ConnectedComponent>::eff_holes(major_connected_component());
    require!(let Option::None = *eff_holes.deleash().matches.index(1 as usize));
    let down_match = *components_max_downwards().deleash().matches.index(0 as usize);
    require!(let Some(_) = down_match);
    let down_match_dp_y = down_match.unwrap().deleash().displacement().y;
    let higher_excess = <crate::connected_component::ConnectedComponent>::upper_mass(major_connected_component()) - <crate::connected_component::ConnectedComponent>::lower_mass(major_connected_component());
    require!(higher_excess > 7.0f32);
    if let Option::None = *eff_holes.deleash().matches.index(0 as usize) {
        require!(major_concave_components().deleash().ilen() >= 2);
        let four_match_refine_result = *components_max_heights().deleash().matches.index(0 as usize);
        require!(let Some(_) = four_match_refine_result);
        require!(<crate::fermi::FermiMatchResult>::norm(components_max_heights()) < 1.0f32);
        let higher_excess = <crate::connected_component::ConnectedComponent>::upper_mass(major_connected_component()) - <crate::connected_component::ConnectedComponent>::lower_mass(major_connected_component());
        let upper_arc = *components_max_heights().deleash().matches.index(0 as usize);
        require!(let Some(_) = upper_arc);
        require!(upper_arc.unwrap().deleash().displacement().y > 0.0f32);
        require!(<crate::line_segment_sketch::concave_component::ConcaveComponent>::angle_change(upper_arc.unwrap()) < -110.0f32);
        require!(<crate::fermi::FermiMatchResult>::norm(components_max_heights()) < 9.0f32);
        let a = major_connected_component().deleash().top_k_row_right_mass_sum(3);
        require!(a < 22.0f32);
        require!(a > 9.0f32);
        return OneVsAll::Yes;
    }
    OneVsAll::Yes
}

#[allow(non_upper_case_globals)]
pub static mut __displacement_downwards__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

#[rustfmt::skip]
pub fn displacement_downwards(cc: Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>) -> Option<f32> {
    let dp = cc.deleash().displacement();
    require!(dp.y < 0.0f32);
    Some(dp.y)
}

#[allow(non_upper_case_globals)]
pub static mut __cc_box_heights__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

#[rustfmt::skip]
pub fn cc_box_heights(cc: Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>) -> Option<f32> {
    let dp = cc.deleash().displacement();
    require!(dp.y > 0.0f32);
    require!(<crate::line_segment_sketch::concave_component::ConcaveComponent>::relative_bounding_box(cc).deleash().ymin() > 0.4f32);
    Some(<crate::line_segment_sketch::concave_component::ConcaveComponent>::relative_bounding_box(cc).deleash().ymin())
}