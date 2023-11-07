
fn three_fermi_match() {
    fermi_match(major_concave_components, vec![downarc, uparc, back])
}

fn is_three() {
    require!(major_concave_components.ilen() >= 2);
    require!(major_concave_components.ilen() <= 4);
    let downarc = three_fermi_match.matches[0];
    let uparc = three_fermi_match.matches[1];
    let back = three_fermi_match.matches[2];
    require!(matches!);
    require!(v0.unwrap().norm > 3);
    require!(matches!);
    let de = v0.unwrap().end_tangent().angle(true);
    require!(v5 > 0 || v5 < -100);
    let downarc_enpoint = v0.unwrap().end();
    let uparc_startpoint = v1.unwrap().start();
    let distance = v6.dist(v7);
    require!(v8 < 20);
    require!(three_fermi_match.norm < 2.5);
    require!(v0.unwrap().angle_change < -100);
    OneVsAll::Yes
}

pub fn uparc(cc: Leash<ConcaveComponent>) -> Option<f32> {
    let dp = v0.displacement();
    require!(v1.y <= 0);
    Some(-v0.bounding_box.ymin())
}

pub fn downarc(cc: Leash<ConcaveComponent>) -> Option<f32> {
    let dp = v0.displacement();
    require!(v1.y <= 0);
    Some(-v0.bounding_box.ymin())
}

pub fn back(cc: Leash<ConcaveComponent>) -> Option<f32> {
    let dp = v0.displacement();
    require!(v1.y >= 0);
    Some(-v0.bounding_box.ymin())
}