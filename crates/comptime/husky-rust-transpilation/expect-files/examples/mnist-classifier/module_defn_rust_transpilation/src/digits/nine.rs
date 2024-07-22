use super::*;

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::val(ingredient_index = 43, return_leash)]
pub fn nine_match() -> crate::fermi::FermiMatchResult {
    crate::fermi::fermi_match(major_concave_components(), vec![downmost])
}

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::val(ingredient_index = 44, return_leash)]
pub fn nine_match_refine() -> crate::fermi::FermiMatchResult {
    crate::fermi::fermi_match(major_concave_components(), vec![big_cc])
}

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::val(ingredient_index = 45)]
pub fn is_nine() -> malamute::OneVsAll {
    let eff_holes = <crate::connected_component::ConnectedComponent>::eff_holes(major_connected_component().deleash());
    require!(let Option::None = eff_holes.deleash().matches[1 as usize]);
    let down_match = nine_match().matches[0 as usize];
    require!(let Some(_) = down_match);
    let down_match_dp_y = down_match.unwrap().deleash().displacement().y;
    let higher_excess = <crate::connected_component::ConnectedComponent>::upper_mass(major_connected_component().deleash()) - <crate::connected_component::ConnectedComponent>::lower_mass(major_connected_component().deleash());
    require!(higher_excess > 7.0f32);
    if let Option::None = eff_holes.deleash().matches[0 as usize] {
        require!(major_concave_components().deleash().ilen() >= 2);
        let nine_match_refine_result = nine_match_refine().matches[0 as usize];
        require!(let Some(_) = nine_match_refine_result);
        require!(<crate::fermi::FermiMatchResult>::norm(nine_match_refine()) < 1.0f32);
        let higher_excess = <crate::connected_component::ConnectedComponent>::upper_mass(major_connected_component().deleash()) - <crate::connected_component::ConnectedComponent>::lower_mass(major_connected_component().deleash());
        let upper_arc = nine_match_refine().matches[0 as usize];
        require!(let Some(_) = upper_arc);
        require!(upper_arc.unwrap().deleash().displacement().y > 0.0f32);
        require!(<crate::line_segment_sketch::concave_component::ConcaveComponent>::angle_change(upper_arc.unwrap().deleash()) < -110.0f32);
        require!(<crate::fermi::FermiMatchResult>::norm(nine_match_refine()) < 9.0f32);
        let a = major_connected_component().deleash().top_k_row_right_mass_sum(3);
        require!(a < 22.0f32);
        require!(a > 9.0f32);
        return OneVsAll::Yes;
    }
    OneVsAll::Yes
}

#[rustfmt::skip]
pub fn downmost(cc: Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>) -> Option<f32> {
    let dp = cc.deleash().displacement();
    require!(dp.y < 0.0f32);
    Some(dp.y)
}

#[rustfmt::skip]
pub fn big_cc(cc: Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>) -> Option<f32> {
    let dp = cc.deleash().displacement();
    require!(dp.y > 0.0f32);
    require!(<crate::line_segment_sketch::concave_component::ConcaveComponent>::relative_bounding_box(cc).ymin() > 0.4f32);
    Some(<crate::line_segment_sketch::concave_component::ConcaveComponent>::relative_bounding_box(cc).ymin())
}