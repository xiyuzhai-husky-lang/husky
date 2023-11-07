
fn one_fermi_match() {
    fermi_match(major_concave_components, vec![downmost, upmost, hat])
}

pub fn upmost(cc: Leash<ConcaveComponent>) -> Option<f32> {
    let dp = v0.displacement();
    require!(v1.y > 0);
    v1.y
}

pub fn downmost(cc: Leash<ConcaveComponent>) -> Option<f32> {
    let dp = v0.displacement();
    require!(v1.y <= 0);
    -v0.end().y
}

pub fn hat(cc: Leash<ConcaveComponent>) -> Option<f32> {
    let dp = v0.displacement();
    require!(v1.y < 0);
    require!(v1.x < 0);
    -v1.y - v1.x
}