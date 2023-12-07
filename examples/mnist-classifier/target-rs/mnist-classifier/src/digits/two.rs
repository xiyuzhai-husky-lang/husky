use super::*;

pub fn two_match() -> FermiMatchResult {
    fermi_match(major_concave_components(), &vec![left_cc_pattern, right_cc_pattern, down_cc_pattern])
}

pub fn left_cc_pattern(cc: Leash<ConcaveComponent>) -> Option<f32> {
    let dp = cc.displacement();
    require!(dp.y < 0.0);
    Some(dp.y)
}

pub fn right_cc_pattern(cc: Leash<ConcaveComponent>) -> Option<f32> {
    let dp = cc.displacement();
    require!(dp.y > 0.0);
    Some(dp.y)
}

pub fn down_cc_pattern(cc: Leash<ConcaveComponent>) -> Option<f32> {
    let dp = cc.displacement();
    require!(dp.x > 0.0);
    Some(dp.x)
}

pub fn is_two() -> OneVsAll {
    require!(let none = is_zero());
    require!(let none = is_three());
    require!(let none = is_seven());
    require!(let none = is_one());
    require!(let none = is_nine());
    require!(let none = is_six());
    let cc_num = major_concave_components().ilen();
    let eff_holes = major_connected_component().eff_holes();
    require!(let none = eff_holes.matches[1]);
    let left_cc = two_match().matches[0];
    let right_cc = two_match().matches[1];
    let down_cc = two_match().matches[2];
    require!(cc_num <= 3);
    let lower_excess = major_connected_component().lower_mass() - major_connected_component().upper_mass();
    require!(lower_excess > 10.0);
    if cc_num == 2 {
        require!(let some = left_cc);
        require!(let some = right_cc);
        let a = right_cc.unwrap().angle_change();
        require!(a > -180.0);
        let end_tan = left_cc.unwrap().end_tangent().angle(true);
        let x = left_cc.unwrap().end_tangent().x;
        let y = left_cc.unwrap().end_tangent().y;
        let left_ymax = left_cc.unwrap().relative_bounding_box().ymax();
        let left_ymin = left_cc.unwrap().relative_bounding_box().ymin();
        let left_mid_y = (left_ymax + left_ymin) / 2.0;
        let right_ymax = right_cc.unwrap().relative_bounding_box().ymax();
        let right_ymin = right_cc.unwrap().relative_bounding_box().ymin();
        let right_mid_y = (right_ymax + right_ymin) / 2.0;
        require!(left_mid_y >= right_mid_y);
    }
    if cc_num == 3 {
        require!(let some = left_cc);
        require!(let some = right_cc);
        require!(let some = down_cc);
        require!(down_cc.unwrap().relative_bounding_box().ymin() < 0.4);
        let a = down_cc.unwrap().angle_change();
    }
    OneVsAll::Yes
}