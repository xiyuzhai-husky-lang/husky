use super::*;

#[rustfmt::skip]
#[ad_hoc_task_dependency::val_item(ingredient_index = 46, return_ref)]
pub fn two_match() -> crate::fermi::FermiMatchResult {
    crate::fermi::fermi_match(major_concave_components(), &vec![left_cc_pattern, right_cc_pattern, down_cc_pattern])
}

#[rustfmt::skip]
pub fn left_cc_pattern(cc: Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>) -> Option<f32> {
    let dp = cc.displacement();
    require!(dp.y < 0.0f32);
    Some(dp.y)
}

#[rustfmt::skip]
pub fn right_cc_pattern(cc: Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>) -> Option<f32> {
    let dp = cc.displacement();
    require!(dp.y > 0.0f32);
    Some(dp.y)
}

#[rustfmt::skip]
pub fn down_cc_pattern(cc: Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>) -> Option<f32> {
    let dp = cc.displacement();
    require!(dp.x > 0.0f32);
    Some(dp.x)
}

#[rustfmt::skip]
#[ad_hoc_task_dependency::val_item(ingredient_index = 47)]
pub fn is_two() -> malamute::OneVsAll {
    let cc_num = major_concave_components().ilen();
    let eff_holes = &major_connected_component().eff_holes();
    require!(let Option::None = eff_holes.matches[1 as usize]);
    let left_cc = two_match().matches[0 as usize];
    let right_cc = two_match().matches[1 as usize];
    let down_cc = two_match().matches[2 as usize];
    require!(cc_num <= 3);
    let lower_excess = major_connected_component().lower_mass() - major_connected_component().upper_mass();
    require!(lower_excess > 10.0f32);
    if cc_num == 2 {
        require!(let Some(_) = left_cc);
        require!(let Some(_) = right_cc);
        let a = right_cc.unwrap().angle_change();
        require!(a > -180.0f32);
        let end_tan = left_cc.unwrap().end_tangent().angle(true);
        let x = left_cc.unwrap().end_tangent().x;
        let y = left_cc.unwrap().end_tangent().y;
        let left_ymax = left_cc.unwrap().relative_bounding_box().ymax();
        let left_ymin = left_cc.unwrap().relative_bounding_box().ymin();
        let left_mid_y = (left_ymax + left_ymin) / 2.0f32;
        let right_ymax = right_cc.unwrap().relative_bounding_box().ymax();
        let right_ymin = right_cc.unwrap().relative_bounding_box().ymin();
        let right_mid_y = (right_ymax + right_ymin) / 2.0f32;
        require!(left_mid_y >= right_mid_y);
    }
    if cc_num == 3 {
        require!(let Some(_) = left_cc);
        require!(let Some(_) = right_cc);
        require!(let Some(_) = down_cc);
        require!(down_cc.unwrap().relative_bounding_box().ymin() < 0.4f32);
        let a = down_cc.unwrap().angle_change();
    }
    OneVsAll::Yes
}