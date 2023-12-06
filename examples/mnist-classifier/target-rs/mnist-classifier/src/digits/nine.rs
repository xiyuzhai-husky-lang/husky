use super::*;

pub fn nine_match() -> FermiMatchResult {
    fermi_match(major_concave_components(), &vec![downmost])
}

pub fn nine_match_refine() -> FermiMatchResult {
    fermi_match(major_concave_components(), &vec![big_cc])
}

pub fn is_nine() -> OneVsAll {
    require!(let none = is_zero());
    require!(let none = is_six());
    let eff_holes = major_connected_component().eff_holes();
    require!(let none = eff_holes.matches[1]);
    let down_match = nine_match().matches[0];
    require!(let some = down_match);
    let down_match_dp_y = down_match.unwrap().displacement().y;
    let higher_excess = major_connected_component().upper_mass() - major_connected_component().lower_mass();
    require!(higher_excess > 7);
    if let none = eff_holes.matches[0] {
        require!(major_concave_components().ilen() >= 2);
        let nine_match_refine_result = nine_match_refine().matches[0];
        require!(let some = nine_match_refine_result);
        require!(nine_match_refine().norm() < 1);
        let higher_excess = major_connected_component().upper_mass() - major_connected_component().lower_mass();
        let upper_arc = nine_match_refine().matches[0];
        require!(let some = upper_arc);
        require!(upper_arc.unwrap().displacement().y > 0);
        require!(upper_arc.unwrap().angle_change() < -110);
        require!(nine_match_refine().norm() < 9);
        let a = major_connected_component().top_k_row_right_mass_sum(3);
        require!(a < 22);
        require!(a > 9);
        return OneVsAll::Yes;
    }
    OneVsAll::Yes
}

pub fn downmost(cc: Leash<ConcaveComponent>) -> Option<f32> {
    let dp = cc.displacement();
    require!(dp.y < 0);
    Some(dp.y)
}

pub fn big_cc(cc: Leash<ConcaveComponent>) -> Option<f32> {
    let dp = cc.displacement();
    require!(dp.y > 0);
    require!(cc.relative_bounding_box().ymin() > 0.4);
    Some(cc.relative_bounding_box().ymin())
}