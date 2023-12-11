use super::*;

#[ad_hoc_task_dependency::val_item(ingredient_index = 26, return_ref)]
pub fn one_fermi_match() -> crate::fermi::FermiMatchResult {
    crate::fermi::fermi_match(major_concave_components(), &vec![downmost, upmost, hat])
}

#[ad_hoc_task_dependency::val_item(ingredient_index = 27, lazy)]
pub fn is_one() -> malamute::OneVsAll {}

pub fn upmost(cc: Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>) -> Option<f32> {
    let dp = cc.displacement();
    require!(dp.y.into_inner() > 0.0f32);
    Some(dp.y.into_inner())
}

pub fn downmost(cc: Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>) -> Option<f32> {
    let dp = cc.displacement();
    require!(dp.y.into_inner() <= 0.0f32);
    Some(-cc.end().y.into_inner())
}

pub fn hat(cc: Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>) -> Option<f32> {
    let dp = cc.displacement();
    require!(dp.y.into_inner() < 0.0f32);
    require!(dp.x.into_inner() < 0.0f32);
    Some(-dp.y.into_inner() - dp.x.into_inner())
}