use super::*;

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::val(ingredient_index = 26, return_leash)]
pub fn one_fermi_match() -> crate::fermi::FermiMatchResult {
    crate::fermi::fermi_match(major_concave_components(), &vec![downmost, upmost, hat])
}

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::val(ingredient_index = 27, lazy)]
pub fn is_one() -> malamute::OneVsAll {}

#[rustfmt::skip]
pub fn upmost(cc: Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>) -> Option<f32> {
    let dp = &cc.deleash().displacement();
    require!(dp.y > 0.0f32);
    Some(dp.y)
}

#[rustfmt::skip]
pub fn downmost(cc: Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>) -> Option<f32> {
    let dp = &cc.deleash().displacement();
    require!(dp.y <= 0.0f32);
    Some(-cc.deleash().end().y)
}

#[rustfmt::skip]
pub fn hat(cc: Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>) -> Option<f32> {
    let dp = &cc.deleash().displacement();
    require!(dp.y < 0.0f32);
    require!(dp.x < 0.0f32);
    Some(-dp.y - dp.x)
}