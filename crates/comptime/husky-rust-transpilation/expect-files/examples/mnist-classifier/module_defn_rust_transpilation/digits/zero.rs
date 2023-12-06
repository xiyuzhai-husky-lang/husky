use super::*;

pub fn open_one_match() -> FermiMatchResult {
    fermi_match(major_concave_components(), &vec![almost_closed])
}

pub fn almost_closed(cc: Leash<ConcaveComponent>) -> Option<f32> {
    require!(cc.angle_change() + 0 < -140);
    -cc.angle_change() + 0
}

