
fn nine_match() {
    fermi_match(major_concave_components, vec![downmost])
}

fn nine_match_refine() {
    fermi_match(major_concave_components, vec![big_cc])
}

fn is_nine() {
    require!(matches!);
    require!(matches!);
    let eff_holes = major_connected_component.eff_holes;
    require!(matches!);
    let down_match = nine_match.matches[0];
    require!(matches!);
    let down_match_dp_y = v4.unwrap().displacement().y;
    let higher_excess = major_connected_component.upper_mass - major_connected_component.lower_mass;
    require!(v7 > 7);
    if matches! {
        require!(major_concave_components.ilen() >= 2);
        let nine_match_refine_result = nine_match_refine.matches[0];
        require!(matches!);
        require!(nine_match_refine.norm < 1);
        let higher_excess = major_connected_component.upper_mass - major_connected_component.lower_mass;
        let upper_arc = nine_match_refine.matches[0];
        require!(matches!);
        require!(v12.unwrap().displacement().y > 0);
        require!(v12.unwrap().angle_change < -110);
        require!(nine_match_refine.norm < 9);
        let a = major_connected_component.top_k_row_right_mass_sum(3);
        require!(v14 < 22);
        require!(v14 > 9);
        return OneVsAll::Yes;
    }
    OneVsAll::Yes
}

pub fn downmost(cc: Leash<ConcaveComponent>) -> Option<f32> {
    let dp = v0.displacement();
    require!(v1.y < 0);
    v1.y
}

pub fn big_cc(cc: Leash<ConcaveComponent>) -> Option<f32> {
    let dp = v0.displacement();
    require!(v1.y > 0);
    require!(v0.relative_bounding_box.ymin() > 0.4);
    v0.relative_bounding_box.ymin()
}