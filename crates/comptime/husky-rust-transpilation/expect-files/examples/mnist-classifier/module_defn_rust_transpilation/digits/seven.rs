
fn simple_seven_match() {
    fermi_match(major_concave_components, vec![simple_leftdown_pattern])
}

pub fn simple_leftdown_pattern(cc: Leash<ConcaveComponent>) -> Option<f32> {
    let dp = cc.displacement();
    require!(dp.y < 0);
    -dp.y
}

fn special_seven_match() {
    fermi_match(major_concave_components, vec![leftupcc_pattern, leftdowncc_pattern])
}

pub fn leftupcc_pattern(cc: Leash<ConcaveComponent>) -> Option<f32> {
    let dp = cc.displacement();
    require!(dp.y < 0);
    require!(cc.relative_bounding_box.ymax() > 0.6);
    cc.end().y
}

pub fn leftdowncc_pattern(cc: Leash<ConcaveComponent>) -> Option<f32> {
    let dp = cc.displacement();
    require!(dp.y < 0);
    require!(cc.relative_bounding_box.ymin() < 0.3);
    let ang = cc.start_tangent().angle(true);
    require!(ang < 30);
    ang
}

fn is_seven() {
    require!(matches!);
    require!(matches!);
    require!(major_connected_component.max_hole_ilen == 0);
    let simple_match_norm = simple_seven_match.norm;
    if simple_match_norm < 1 {
        require!(matches!);
        let upper_excess = major_connected_component.upper_mass - major_connected_component.lower_mass;
        if upper_excess < 10 {
            let end_tangent = simple_seven_match.matches[0].unwrap().end_tangent();
            let a = end_tangent.y;
            require!(a < -7);
        }
        return OneVsAll::Yes;
    }
    if simple_match_norm < 4 {
        let upper_excess = major_connected_component.upper_mass - major_connected_component.lower_mass;
        require!(upper_excess > 10);
        return OneVsAll::Yes;
    }
    require!(matches!);
    let others = special_seven_match.others;
    require!(false);
    OneVsAll::Yes
}