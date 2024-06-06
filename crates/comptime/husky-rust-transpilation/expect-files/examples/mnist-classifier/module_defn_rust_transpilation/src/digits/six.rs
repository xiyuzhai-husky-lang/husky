use super::*;

#[rustfmt::skip]
#[ad_hoc_task_dependency::val_item(ingredient_index = 28, return_ref)]
pub fn six_match() -> crate::fermi::FermiMatchResult {
    crate::fermi::fermi_match(major_concave_components(), &vec![upmost])
}

#[rustfmt::skip]
#[ad_hoc_task_dependency::val_item(ingredient_index = 29, return_ref)]
pub fn six_match_refined1() -> crate::fermi::FermiMatchResult {
    crate::fermi::fermi_match(major_concave_components(), &vec![upmost, bottom1])
}

#[rustfmt::skip]
#[ad_hoc_task_dependency::val_item(ingredient_index = 30, lazy)]
pub fn is_six() -> malamute::OneVsAll {}

#[rustfmt::skip]
pub fn upmost(cc: Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>) -> Option<f32> {
    let dp = cc.displacement();
    require!(dp.y > 0.0f32);
    Some(dp.y)
}

#[rustfmt::skip]
pub fn bottom1(cc: Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>) -> Option<f32> {
    let dp = cc.displacement();
    if dp.y < -3.0f32 {
        require!((dp.x / dp.y).abs() > 1.4f32);
    }
    require!(cc.relative_bounding_box().ymax() < 0.6f32);
    let relative_end = cc.line_segment_sketch.bounding_box().relative_point(&cc.end());
    require!(relative_end.x > 0.5f32);
    Some(-cc.end().y)
}