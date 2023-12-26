use super::*;

#[rustfmt::skip]
#[ad_hoc_task_dependency::val_item(ingredient_index = 41, return_ref)]
pub fn upper_mouth_match() -> crate::fermi::FermiMatchResult {
    crate::fermi::fermi_match(major_concave_components(), &vec![big_mouth])
}

#[rustfmt::skip]
#[ad_hoc_task_dependency::val_item(ingredient_index = 42)]
pub fn is_eight() -> malamute::OneVsAll {
    let upper_excess = major_connected_component().upper_mass() - major_connected_component().lower_mass();
    if let Option::None = major_connected_component().eff_holes().matches[1 as usize] {
        if let Option::None = major_connected_component().eff_holes().matches[0 as usize] {
            require!(false);
        }
        require!(false);
    }
    OneVsAll::Yes
}

#[rustfmt::skip]
pub fn big_mouth(cc: Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>) -> Option<f32> {
    if cc.relative_bounding_box().ymax() > 0.5f32 {
        require!(cc.strokes.first().unwrap().start.x.into_inner() > cc.strokes.first().unwrap().end.x.into_inner());
    }
    Some(cc.relative_bounding_box().ymax())
}
