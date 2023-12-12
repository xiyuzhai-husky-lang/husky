use super::*;

#[rustfmt::skip]
#[ad_hoc_task_dependency::val_item(ingredient_index = 43, return_ref)]
pub fn nine_match() -> crate::fermi::FermiMatchResult {
    crate::fermi::fermi_match(major_concave_components(), &vec![downmost])
}

#[rustfmt::skip]
#[ad_hoc_task_dependency::val_item(ingredient_index = 44, return_ref)]
pub fn nine_match_refine() -> crate::fermi::FermiMatchResult {
    crate::fermi::fermi_match(major_concave_components(), &vec![big_cc])
}

#[rustfmt::skip]
#[ad_hoc_task_dependency::val_item(ingredient_index = 45)]
pub fn is_nine() -> malamute::OneVsAll {
    require!(let none = is_zero());
    require!(let none = is_six());
    let eff_holes = &major_connected_component().eff_holes();
    require!(let none = eff_holes.matches[1 as usize]);
    let down_match = nine_match().matches[0 as usize];
    require!(let some = down_match);
    let down_match_dp_y = down_match.unwrap().displacement().y.into_inner();
    let higher_excess = major_connected_component().upper_mass() - major_connected_component().lower_mass();
    require!(higher_excess > 7.0f32);
    if let none = eff_holes.matches[0 as usize] {
        require!(major_concave_components().ilen() >= 2);
        let nine_match_refine_result = nine_match_refine().matches[0 as usize];
        require!(let some = nine_match_refine_result);
        require!(nine_match_refine().norm() < 1.0f32);
        let higher_excess = major_connected_component().upper_mass() - major_connected_component().lower_mass();
        let upper_arc = nine_match_refine().matches[0 as usize];
        require!(let some = upper_arc);
        require!(upper_arc.unwrap().displacement().y.into_inner() > 0.0f32);
        require!(upper_arc.unwrap().angle_change() < -110.0f32);
        require!(nine_match_refine().norm() < 9.0f32);
        let a = major_connected_component().top_k_row_right_mass_sum(3);
        require!(a < 22.0f32);
        require!(a > 9.0f32);
        return OneVsAll::Yes;
    }
    OneVsAll::Yes
}

#[rustfmt::skip]
pub fn downmost(cc: Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>) -> Option<f32> {
    let dp = cc.displacement();
    require!(dp.y.into_inner() < 0.0f32);
    Some(dp.y.into_inner())
}

#[rustfmt::skip]
pub fn big_cc(cc: Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>) -> Option<f32> {
    let dp = cc.displacement();
    require!(dp.y.into_inner() > 0.0f32);
    require!(cc.relative_bounding_box().ymin() > 0.4f32);
    Some(cc.relative_bounding_box().ymin())
}