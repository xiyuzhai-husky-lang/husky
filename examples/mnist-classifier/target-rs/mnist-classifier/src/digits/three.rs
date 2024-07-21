use super::*;

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::val(ingredient_index = 31, return_leash)]
pub fn three_fermi_match() -> crate::fermi::FermiMatchResult {
    crate::fermi::fermi_match(major_concave_components(), &vec![downarc, uparc, back])
}

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::val(ingredient_index = 32)]
pub fn is_three() -> malamute::OneVsAll {
    require!(major_concave_components().deleash().ilen() >= 2);
    require!(major_concave_components().deleash().ilen() <= 4);
    let downarc = three_fermi_match().matches[0 as usize];
    let uparc = three_fermi_match().matches[1 as usize];
    let back = three_fermi_match().matches[2 as usize];
    require!(let Some(_) = downarc);
    require!(<crate::line_segment_sketch::concave_component::ConcaveComponent>::norm(downarc.unwrap()) > 3.0f32);
    require!(let Some(_) = uparc);
    let de = downarc.unwrap().deleash().end_tangent().angle(true);
    require!(de > 0.0f32 || de < -100.0f32);
    let downarc_enpoint = downarc.unwrap().deleash().end();
    let uparc_startpoint = uparc.unwrap().deleash().start();
    let distance = downarc_enpoint.dist(&uparc_startpoint);
    require!(distance < 20.0f32);
    require!(<crate::fermi::FermiMatchResult>::norm(Leash(&three_fermi_match())) < 2.5f32);
    require!(<crate::line_segment_sketch::concave_component::ConcaveComponent>::angle_change(downarc.unwrap()) < -100.0f32);
    OneVsAll::Yes
}

#[rustfmt::skip]
pub fn uparc(cc: Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>) -> Option<f32> {
    let dp = cc.deleash().displacement();
    require!(dp.y <= 0.0f32);
    Option::Some(-<crate::line_segment_sketch::concave_component::ConcaveComponent>::bounding_box(cc).ymin())
}

#[rustfmt::skip]
pub fn downarc(cc: Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>) -> Option<f32> {
    let dp = cc.deleash().displacement();
    require!(dp.y <= 0.0f32);
    Option::Some(-<crate::line_segment_sketch::concave_component::ConcaveComponent>::bounding_box(cc).ymin())
}

#[rustfmt::skip]
pub fn back(cc: Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>) -> Option<f32> {
    let dp = cc.deleash().displacement();
    require!(dp.y >= 0.0f32);
    Option::Some(-<crate::line_segment_sketch::concave_component::ConcaveComponent>::bounding_box(cc).ymin())
}