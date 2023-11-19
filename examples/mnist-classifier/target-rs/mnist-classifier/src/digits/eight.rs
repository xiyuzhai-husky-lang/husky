use super::*;

pub fn upper_mouth_match() -> FermiMatchResult {
    fermi_match(major_concave_components, vec![big_mouth])
}

pub fn is_eight() -> OneVsAll<MnistLabel> {
    require!(matches!);
    require!(matches!);
    require!(matches!);
    require!(matches!);
    let upper_excess = major_connected_component.upper_mass - major_connected_component.lower_mass;
    if matches! {
        if matches! {
            require!(false);
        }
        require!(false);
    }
    OneVsAll::Yes
}

pub fn big_mouth(cc: Leash<ConcaveComponent>) -> Option<f32> {
    if cc.relative_bounding_box.ymax() > 0.5 {
        require!(cc.strokes.first().unwrap().start.x > cc.strokes.first().unwrap().end.x);
    }
    cc.relative_bounding_box.ymax()
}