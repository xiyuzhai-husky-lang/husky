use super::*;

pub fn six_match() -> FermiMatchResult {
    fermi_match(major_concave_components(), &vec![upmost])
}

pub fn six_match_refined1() -> FermiMatchResult {
    fermi_match(major_concave_components(), &vec![upmost, bottom1])
}

pub fn upmost(cc: Leash<ConcaveComponent>) -> Option<f32> {
    let dp = cc.displacement();
    require!(dp.y > 0.0f32);
    Some(dp.y)
}

pub fn bottom1(cc: Leash<ConcaveComponent>) -> Option<f32> {
    let dp = cc.displacement();
    if dp.y < -3.0f32 {
        require!((dp.x / dp.y).abs() > 1.4f32);
    }
    require!(cc.relative_bounding_box().ymax() < 0.6f32);
    let relative_end = cc.line_segment_sketch.bounding_box().relative_point(&cc.end());
    require!(relative_end.x > 0.5f32);
    Some(-cc.end().y)
}