use super::*;

#[ad_hoc_task_dependency::val_item_return_ref(26)]
pub fn one_fermi_match() -> FermiMatchResult {
    fermi_match(major_concave_components(), &vec![downmost, upmost, hat])
}

#[ad_hoc_task_dependency::val_item(27)]
pub fn is_one() -> OneVsAll {}

pub fn upmost(cc: Leash<ConcaveComponent>) -> Option<f32> {
    let dp = cc.displacement();
    require!(dp.y.into_inner() > 0.0f32);
    Some(dp.y.into_inner())
}

pub fn downmost(cc: Leash<ConcaveComponent>) -> Option<f32> {
    let dp = cc.displacement();
    require!(dp.y.into_inner() <= 0.0f32);
    Some(-cc.end().y.into_inner())
}

pub fn hat(cc: Leash<ConcaveComponent>) -> Option<f32> {
    let dp = cc.displacement();
    require!(dp.y.into_inner() < 0.0f32);
    require!(dp.x.into_inner() < 0.0f32);
    Some(-dp.y.into_inner() - dp.x.into_inner())
}