
fn left_components() {
    fermi_match(major_concave_components, vec![left_coordinate_max, left_coordinate_max])
}

pub fn left_coordinate_max(cc: Leash<ConcaveComponent>) -> Option<f32> {
    v0.relative_bounding_box.xmax()
}

fn components_max_downwards() {
    fermi_match(major_concave_components, vec![displacement_downwards])
}

fn components_max_heights() {
    fermi_match(major_concave_components, vec![cc_box_heights])
}

fn is_four() {
    require!(matches!);
    require!(matches!);
    let eff_holes = major_connected_component.eff_holes;
    require!(matches!);
    let down_match = components_max_downwards.matches[0];
    require!(matches!);
    let down_match_dp_y = v4.unwrap().displacement().y;
    let higher_excess = major_connected_component.upper_mass - major_connected_component.lower_mass;
    require!(v7 > 7);
    if matches! {
        require!(major_concave_components.ilen() >= 2);
        let four_match_refine_result = components_max_heights.matches[0];
        require!(matches!);
        require!(components_max_heights.norm < 1);
        let higher_excess = major_connected_component.upper_mass - major_connected_component.lower_mass;
        let upper_arc = components_max_heights.matches[0];
        require!(matches!);
        require!(v12.unwrap().displacement().y > 0);
        require!(v12.unwrap().angle_change < -110);
        require!(components_max_heights.norm < 9);
        let a = major_connected_component.top_k_row_right_mass_sum(3);
        require!(v14 < 22);
        require!(v14 > 9);
        return OneVsAll::Yes;
    }
    OneVsAll::Yes
}

pub fn displacement_downwards(cc: Leash<ConcaveComponent>) -> Option<f32> {
    let dp = v0.displacement();
    require!(v1.y < 0);
    v1.y
}

pub fn cc_box_heights(cc: Leash<ConcaveComponent>) -> Option<f32> {
    let dp = v0.displacement();
    require!(v1.y > 0);
    require!(v0.relative_bounding_box.ymin() > 0.4);
    v0.relative_bounding_box.ymin()
}