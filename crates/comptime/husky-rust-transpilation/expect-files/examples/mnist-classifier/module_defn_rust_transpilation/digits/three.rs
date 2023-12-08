use super::*;

pub fn three_fermi_match() -> FermiMatchResult {
    fermi_match(major_concave_components(), (&vec![downarc, uparc, back]))
}

pub fn is_three() -> OneVsAll {
    require!(major_concave_components().ilen() >= 2);
    require!(major_concave_components().ilen() <= 4);
    let downarc = three_fermi_match().matches[0 as usize];
    let uparc = three_fermi_match().matches[1 as usize];
    let back = three_fermi_match().matches[2 as usize];
    require!(let some = downarc);
    require!(downarc.unwrap().norm() > 3.0f32);
    require!(let some = uparc);
    let de = downarc.unwrap().end_tangent().angle(true);
    require!(de > 0.0f32 || de < -100.0f32);
    let downarc_enpoint = downarc.unwrap().end();
    let uparc_startpoint = uparc.unwrap().start();
    let distance = downarc_enpoint.dist((&uparc_startpoint));
    require!(distance < 20.0f32);
    require!(three_fermi_match().norm() < 2.5f32);
    require!(downarc.unwrap().angle_change() < -100.0f32);
    OneVsAll::Yes
}

pub fn uparc(cc: Leash<ConcaveComponent>) -> Option<f32> {
    let dp = cc.displacement();
    require!(dp.y.into_inner() <= 0.0f32);
    Option::Some(-cc.bounding_box().ymin())
}

pub fn downarc(cc: Leash<ConcaveComponent>) -> Option<f32> {
    let dp = cc.displacement();
    require!(dp.y.into_inner() <= 0.0f32);
    Option::Some(-cc.bounding_box().ymin())
}

pub fn back(cc: Leash<ConcaveComponent>) -> Option<f32> {
    let dp = cc.displacement();
    require!(dp.y.into_inner() >= 0.0f32);
    Option::Some(-cc.bounding_box().ymin())
}