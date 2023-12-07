use super::*;

pub fn left_components() -> FermiMatchResult {
    fermi_match(major_concave_components(), &vec![left_coordinate_max, left_coordinate_max])
}

pub fn left_coordinate_max(cc: Leash<ConcaveComponent>) -> Option<f32> {
    Some(cc.relative_bounding_box().xmax())
}

pub fn components_max_downwards() -> FermiMatchResult {
    fermi_match(major_concave_components(), &vec![displacement_downwards])
}

pub fn components_max_heights() -> FermiMatchResult {
    fermi_match(major_concave_components(), &vec![cc_box_heights])
}

pub fn is_four() -> OneVsAll {
    require!(let some = left_components().matches[0]);
    require!(let some = left_components().matches[1]);
    let eff_holes = major_connected_component().eff_holes();
    require!(let none = eff_holes.matches[1]);
    let down_match = components_max_downwards().matches[0];
    require!(let some = down_match);
    let down_match_dp_y = down_match.unwrap().displacement().y;
    let higher_excess = major_connected_component().upper_mass() - major_connected_component().lower_mass();
    require!(higher_excess > 7.0);
    if let none = eff_holes.matches[0] {
        require!(major_concave_components().ilen() >= 2);
        let four_match_refine_result = components_max_heights().matches[0];
        require!(let some = four_match_refine_result);
        require!(components_max_heights().norm() < 1.0);
        let higher_excess = major_connected_component().upper_mass() - major_connected_component().lower_mass();
        let upper_arc = components_max_heights().matches[0];
        require!(let some = upper_arc);
        require!(upper_arc.unwrap().displacement().y > 0.0);
        require!(upper_arc.unwrap().angle_change() < -110.0);
        require!(components_max_heights().norm() < 9.0);
        let a = major_connected_component().top_k_row_right_mass_sum(3);
        require!(a < 22.0);
        require!(a > 9.0);
        return OneVsAll::Yes;
    }
    OneVsAll::Yes
}

pub fn displacement_downwards(cc: Leash<ConcaveComponent>) -> Option<f32> {
    let dp = cc.displacement();
    require!(dp.y < 0.0);
    Some(dp.y)
}

pub fn cc_box_heights(cc: Leash<ConcaveComponent>) -> Option<f32> {
    let dp = cc.displacement();
    require!(dp.y > 0.0);
    require!(cc.relative_bounding_box().ymin() > 0.4);
    Some(cc.relative_bounding_box().ymin())
}