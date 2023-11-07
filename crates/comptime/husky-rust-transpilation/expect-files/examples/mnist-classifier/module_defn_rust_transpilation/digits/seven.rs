
fn simple_seven_match() {
    fermi_match(major_concave_components, vec![simple_leftdown_pattern])
}

pub fn simple_leftdown_pattern(cc: Leash<ConcaveComponent>) -> Option<f32> {
    let dp = v0.displacement();
    require!(v1.y < 0);
    -v1.y
}

fn special_seven_match() {
    fermi_match(major_concave_components, vec![leftupcc_pattern, leftdowncc_pattern])
}

pub fn leftupcc_pattern(cc: Leash<ConcaveComponent>) -> Option<f32> {
    let dp = v0.displacement();
    require!(v1.y < 0);
    require!(v0.relative_bounding_box.ymax() > 0.6);
    v0.end().y
}

pub fn leftdowncc_pattern(cc: Leash<ConcaveComponent>) -> Option<f32> {
    let dp = v0.displacement();
    require!(v1.y < 0);
    require!(v0.relative_bounding_box.ymin() < 0.3);
    let ang = v0.start_tangent().angle(true);
    require!(v2 < 30);
    v2
}

fn is_seven() {
    require!(matches!);
    require!(matches!);
    require!(major_connected_component.max_hole_ilen == 0);
    let simple_match_norm = simple_seven_match.norm;
    if v2 < 1 {
        require!(matches!);
        let upper_excess = major_connected_component.upper_mass - major_connected_component.lower_mass;
        if v4 < 10 {
            let end_tangent = simple_seven_match.matches[0].unwrap().end_tangent();
            let a = v5.y;
            require!(v6 < -7);
        }
        return OneVsAll::Yes;
    }
    if v2 < 4 {
        let upper_excess = major_connected_component.upper_mass - major_connected_component.lower_mass;
        require!(v7 > 10);
        return OneVsAll::Yes;
    }
    require!(matches!);
    let others = special_seven_match.others;
    require!(false);
    OneVsAll::Yes
}