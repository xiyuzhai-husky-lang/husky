use super::*;

pub static mut __two_match__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::val(item_path_id_interface = __two_match__ITEM_PATH_ID_INTERFACE, return_leash)]
pub fn two_match() -> crate::fermi::FermiMatchResult {
    crate::fermi::fermi_match(major_concave_components(), &vec![left_cc_pattern, right_cc_pattern, down_cc_pattern])
}

pub static mut __left_cc_pattern__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

#[rustfmt::skip]
pub fn left_cc_pattern(cc: Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>) -> Option<f32> {
    let dp = cc.deleash().displacement();
    require!(dp.y < 0.0f32);
    Some(dp.y)
}

pub static mut __right_cc_pattern__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

#[rustfmt::skip]
pub fn right_cc_pattern(cc: Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>) -> Option<f32> {
    let dp = cc.deleash().displacement();
    require!(dp.y > 0.0f32);
    Some(dp.y)
}

pub static mut __down_cc_pattern__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

#[rustfmt::skip]
pub fn down_cc_pattern(cc: Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>) -> Option<f32> {
    let dp = cc.deleash().displacement();
    require!(dp.x > 0.0f32);
    Some(dp.x)
}

pub static mut __is_two__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::val(item_path_id_interface = __is_two__ITEM_PATH_ID_INTERFACE)]
pub fn is_two() -> malamute::OneVsAll {
    let cc_num = major_concave_components().deleash().ilen();
    let eff_holes = <crate::connected_component::ConnectedComponent>::eff_holes(major_connected_component());
    require!(let Option::None = eff_holes.deleash().matches[1 as usize]);
    let left_cc = two_match().deleash().matches[0 as usize];
    let right_cc = two_match().deleash().matches[1 as usize];
    let down_cc = two_match().deleash().matches[2 as usize];
    require!(cc_num <= 3);
    let lower_excess = <crate::connected_component::ConnectedComponent>::lower_mass(major_connected_component()) - <crate::connected_component::ConnectedComponent>::upper_mass(major_connected_component());
    require!(lower_excess > 10.0f32);
    if cc_num == 2 {
        require!(let Some(_) = left_cc);
        require!(let Some(_) = right_cc);
        let a = <crate::line_segment_sketch::concave_component::ConcaveComponent>::angle_change(right_cc.unwrap());
        require!(a > -180.0f32);
        let end_tan = left_cc.unwrap().deleash().end_tangent().angle(true);
        let x = left_cc.unwrap().deleash().end_tangent().x;
        let y = left_cc.unwrap().deleash().end_tangent().y;
        let left_ymax = <crate::line_segment_sketch::concave_component::ConcaveComponent>::relative_bounding_box(left_cc.unwrap()).deleash().ymax();
        let left_ymin = <crate::line_segment_sketch::concave_component::ConcaveComponent>::relative_bounding_box(left_cc.unwrap()).deleash().ymin();
        let left_mid_y = (left_ymax + left_ymin) / 2.0f32;
        let right_ymax = <crate::line_segment_sketch::concave_component::ConcaveComponent>::relative_bounding_box(right_cc.unwrap()).deleash().ymax();
        let right_ymin = <crate::line_segment_sketch::concave_component::ConcaveComponent>::relative_bounding_box(right_cc.unwrap()).deleash().ymin();
        let right_mid_y = (right_ymax + right_ymin) / 2.0f32;
        require!(left_mid_y >= right_mid_y);
    }
    if cc_num == 3 {
        require!(let Some(_) = left_cc);
        require!(let Some(_) = right_cc);
        require!(let Some(_) = down_cc);
        require!(<crate::line_segment_sketch::concave_component::ConcaveComponent>::relative_bounding_box(down_cc.unwrap()).deleash().ymin() < 0.4f32);
        let a = <crate::line_segment_sketch::concave_component::ConcaveComponent>::angle_change(down_cc.unwrap());
    }
    OneVsAll::Yes
}