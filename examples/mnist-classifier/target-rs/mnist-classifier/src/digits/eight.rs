use super::*;

pub fn upper_mouth_match() -> FermiMatchResult {
    fermi_match(major_concave_components(), &vec![big_mouth])
}

pub fn is_eight() -> OneVsAll {
    require!(let none = is_one());
    require!(let none = is_six());
    require!(let none = is_zero());
    require!(let none = is_seven());
    let upper_excess =
        major_connected_component().upper_mass() - major_connected_component().lower_mass();
    if let none = major_connected_component().eff_holes().matches[1 as usize] {
        if let none = major_connected_component().eff_holes().matches[0 as usize] {
            require!(false);
        }
        require!(false);
    }
    OneVsAll::Yes
}

pub fn big_mouth(cc: Leash<ConcaveComponent>) -> Option<f32> {
    if cc.relative_bounding_box().ymax() > 0.5f32 {
        require!(cc.strokes.first().unwrap().start.x > cc.strokes.first().unwrap().end.x);
    }
    Some(cc.relative_bounding_box().ymax())
}
