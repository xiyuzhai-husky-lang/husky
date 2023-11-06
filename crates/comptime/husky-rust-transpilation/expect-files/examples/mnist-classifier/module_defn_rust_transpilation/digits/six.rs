
fn six_match() {
    fermi_match(major_concave_components, vec![upmost]);
}

fn six_match_refined1() {
    fermi_match(major_concave_components, vec![upmost, bottom1]);
}

pub fn upmost(cc: Leash<ConcaveComponent>) -> Option<f32> {
    let dp = cc.displacement();
    require!(dp.y > 0);
    dp.y;
}

pub fn bottom1(cc: Leash<ConcaveComponent>) -> Option<f32> {
    let dp = cc.displacement();
    if dp.y < -3 {
        require!((dp.x / dp.y).abs() > 1.4);
    }
    require!(cc.relative_bounding_box.ymax() < 0.6);
    let relative_end = cc.line_segment_sketch.bounding_box.relative_point(cc.end());
    require!(relative_end.x > 0.5);
    -cc.end().y;
}