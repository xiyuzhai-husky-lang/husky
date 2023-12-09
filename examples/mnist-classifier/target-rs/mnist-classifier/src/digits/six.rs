use super::*;

#[ad_hoc_task_dependency::val_item(ingredient_index = 28, return_ref)]
pub fn six_match() -> FermiMatchResult {
    fermi_match(major_concave_components(), &vec![upmost])
}

#[ad_hoc_task_dependency::val_item(ingredient_index = 29, return_ref)]
pub fn six_match_refined1() -> FermiMatchResult {
    fermi_match(major_concave_components(), &vec![upmost, bottom1])
}

#[ad_hoc_task_dependency::val_item(ingredient_index = 30, lazy)]
pub fn is_six() -> OneVsAll {}

pub fn upmost(cc: Leash<ConcaveComponent>) -> Option<f32> {
    let dp = cc.displacement();
    require!(dp.y.into_inner() > 0.0f32);
    Some(dp.y.into_inner())
}

pub fn bottom1(cc: Leash<ConcaveComponent>) -> Option<f32> {
    let dp = cc.displacement();
    if dp.y.into_inner() < -3.0f32 {
        require!((dp.x.into_inner() / dp.y.into_inner()).abs() > 1.4f32);
    }
    require!(cc.relative_bounding_box().ymax() < 0.6f32);
    let relative_end = cc.line_segment_sketch.bounding_box().relative_point(&cc.end());
    require!(relative_end.x.into_inner() > 0.5f32);
    Some(-cc.end().y.into_inner())
}