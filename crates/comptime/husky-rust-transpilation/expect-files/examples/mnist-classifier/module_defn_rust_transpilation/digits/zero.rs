
fn open_one_match() {
    fermi_match(major_concave_components, vec![almost_closed])
}

pub fn almost_closed(cc: Leash<ConcaveComponent>) -> Option<f32> {
    require!(v0.angle_change + 0 < -140);
    -v0.angle_change + 0
}

