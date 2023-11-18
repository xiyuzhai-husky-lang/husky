
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
    require!(downarc.unwrap().norm > 3);
    require!(matches!);
    let de = downarc.unwrap().end_tangent().angle(true);
    require!(de > 0 || de < -100);
    let downarc_enpoint = downarc.unwrap().end();
    let uparc_startpoint = uparc.unwrap().start();
    let distance = downarc_enpoint.dist(uparc_startpoint);
    require!(distance < 20);
    require!(three_fermi_match.norm < 2.5);
    require!(downarc.unwrap().angle_change < -100);
    OneVsAll::Yes
}

pub fn uparc(cc: Leash<ConcaveComponent>) -> Option<f32> {
    let dp = cc.displacement();
    require!(dp.y <= 0);
    Some(-cc.bounding_box.ymin())
}

pub fn downarc(cc: Leash<ConcaveComponent>) -> Option<f32> {
    let dp = cc.displacement();
    require!(dp.y <= 0);
    Some(-cc.bounding_box.ymin())
}

pub fn back(cc: Leash<ConcaveComponent>) -> Option<f32> {
    let dp = cc.displacement();
    require!(dp.y >= 0);
    Some(-cc.bounding_box.ymin())
}