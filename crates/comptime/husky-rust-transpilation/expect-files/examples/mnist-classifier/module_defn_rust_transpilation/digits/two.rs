
fn two_match() {
    fermi_match(major_concave_components, vec![left_cc_pattern, right_cc_pattern, down_cc_pattern])
}

pub fn left_cc_pattern(cc: Leash<ConcaveComponent>) -> Option<f32> {
    let dp = v0.displacement();
    require!(v1.y < 0);
    v1.y
}

pub fn right_cc_pattern(cc: Leash<ConcaveComponent>) -> Option<f32> {
    let dp = v0.displacement();
    require!(v1.y > 0);
    v1.y
}

pub fn down_cc_pattern(cc: Leash<ConcaveComponent>) -> Option<f32> {
    let dp = v0.displacement();
    require!(v1.x > 0);
    v1.x
}

fn is_two() {
    require!(matches!);
    require!(matches!);
    require!(matches!);
    require!(matches!);
    require!(matches!);
    require!(matches!);
    let cc_num = major_concave_components.ilen();
    let eff_holes = major_connected_component.eff_holes;
    require!(matches!);
    let left_cc = two_match.matches[0];
    let right_cc = two_match.matches[1];
    let down_cc = two_match.matches[2];
    require!(v6 <= 3);
    let lower_excess = major_connected_component.lower_mass - major_connected_component.upper_mass;
    require!(v12 > 10);
    if v6 == 2 {
        require!(matches!);
        require!(matches!);
        let a = v10.unwrap().angle_change;
        require!(v15 > -180);
        let end_tan = v9.unwrap().end_tangent().angle(true);
        let x = v9.unwrap().end_tangent().x;
        let y = v9.unwrap().end_tangent().y;
        let left_ymax = v9.unwrap().relative_bounding_box.ymax();
        let left_ymin = v9.unwrap().relative_bounding_box.ymin();
        let left_mid_y = (v19 + v20) / 2;
        let right_ymax = v10.unwrap().relative_bounding_box.ymax();
        let right_ymin = v10.unwrap().relative_bounding_box.ymin();
        let right_mid_y = (v22 + v23) / 2;
        require!(v21 >= v24);
    }
    if v6 == 3 {
        require!(matches!);
        require!(matches!);
        require!(matches!);
        require!(v11.unwrap().relative_bounding_box.ymin() < 0.4);
        let a = v11.unwrap().angle_change;
    }
    OneVsAll::Yes
}