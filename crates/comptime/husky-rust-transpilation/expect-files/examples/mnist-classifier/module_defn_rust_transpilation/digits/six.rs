
fn six_match() {
    fermi_match(major_concave_components, vec![upmost])
}

fn six_match_refined1() {
    fermi_match(major_concave_components, vec![upmost, bottom1])
}

pub fn upmost(cc: Leash<ConcaveComponent>) -> Option<f32> {
    let dp = v0.displacement();
    require!(v1.y > 0);
    v1.y
}

pub fn bottom1(cc: Leash<ConcaveComponent>) -> Option<f32> {
    let dp = v0.displacement();
    if v1.y < -3 {
        require!((v1.x / v1.y).abs() > 1.4);
    }
    require!(v0.relative_bounding_box.ymax() < 0.6);
    let relative_end = v0.line_segment_sketch.bounding_box.relative_point(v0.end());
    require!(v2.x > 0.5);
    -v0.end().y
}