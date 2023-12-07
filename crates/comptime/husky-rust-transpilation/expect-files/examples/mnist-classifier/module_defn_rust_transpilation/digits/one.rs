use super::*;

pub fn one_fermi_match() -> FermiMatchResult {
    fermi_match(major_concave_components(), &vec![downmost, upmost, hat])
}

pub fn upmost(cc: Leash<ConcaveComponent>) -> Option<f32> {
    let dp = cc.displacement();
    require!(dp.y > 0.0);
    Some(dp.y)
}

pub fn downmost(cc: Leash<ConcaveComponent>) -> Option<f32> {
    let dp = cc.displacement();
    require!(dp.y <= 0.0);
    Some(-cc.end().y)
}

pub fn hat(cc: Leash<ConcaveComponent>) -> Option<f32> {
    let dp = cc.displacement();
    require!(dp.y < 0.0);
    require!(dp.x < 0.0);
    Some(-dp.y - dp.x)
}