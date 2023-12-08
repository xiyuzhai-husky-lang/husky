use super::*;

pub fn simple_seven_match() -> FermiMatchResult {
    fermi_match(major_concave_components(), (&vec![simple_leftdown_pattern]))
}

pub fn simple_leftdown_pattern(cc: Leash<ConcaveComponent>) -> Option<f32> {
    let dp = cc.displacement();
    require!(dp.y.into_inner() < 0.0f32);
    (Some(-dp.y.into_inner()))
}

pub fn special_seven_match() -> FermiMatchResult {
    fermi_match(major_concave_components(), (&vec![leftupcc_pattern, leftdowncc_pattern]))
}

pub fn leftupcc_pattern(cc: Leash<ConcaveComponent>) -> Option<f32> {
    let dp = cc.displacement();
    require!(dp.y.into_inner() < 0.0f32);
    require!(cc.relative_bounding_box().ymax() > 0.6f32);
    (Some(cc.end().y.into_inner()))
}

pub fn leftdowncc_pattern(cc: Leash<ConcaveComponent>) -> Option<f32> {
    let dp = cc.displacement();
    require!(dp.y.into_inner() < 0.0f32);
    require!(cc.relative_bounding_box().ymin() < 0.3f32);
    let ang = cc.start_tangent().angle(true);
    require!(ang < 30.0f32);
    (Some(ang))
}

pub fn is_seven() -> OneVsAll {
    require!(let none = is_six());
    require!(let none = is_zero());
    require!(major_connected_component().max_hole_ilen() == 0.0f32);
    let simple_match_norm = simple_seven_match().norm();
    if simple_match_norm < 1.0f32 {
        require!(let some = simple_seven_match().matches[0 as usize]);
        let upper_excess = major_connected_component().upper_mass() - major_connected_component().lower_mass();
        if upper_excess < 10.0f32 {
            let end_tangent = simple_seven_match().matches[0 as usize].unwrap().end_tangent();
            let a = end_tangent.y.into_inner();
            require!(a < -7.0f32);
        }
        return OneVsAll::Yes;
    }
    if simple_match_norm < 4.0f32 {
        let upper_excess = major_connected_component().upper_mass() - major_connected_component().lower_mass();
        require!(upper_excess > 10.0f32);
        return OneVsAll::Yes;
    }
    require!(let some = special_seven_match().matches[0 as usize]);
    let others = special_seven_match().others;
    require!(false);
    OneVsAll::Yes
}