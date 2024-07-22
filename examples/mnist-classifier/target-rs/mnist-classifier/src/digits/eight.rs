use super::*;

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::val(ingredient_index = 41, return_leash)]
pub fn upper_mouth_match() -> crate::fermi::FermiMatchResult {
    crate::fermi::fermi_match(major_concave_components(), &vec![big_mouth])
}

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::val(ingredient_index = 42)]
pub fn is_eight() -> malamute::OneVsAll {
    let upper_excess = <crate::connected_component::ConnectedComponent>::upper_mass(major_connected_component()) - <crate::connected_component::ConnectedComponent>::lower_mass(major_connected_component());
    if let Option::None = <crate::connected_component::ConnectedComponent>::eff_holes(major_connected_component()).deleash().matches[1 as usize] {
        if let Option::None = <crate::connected_component::ConnectedComponent>::eff_holes(major_connected_component()).deleash().matches[0 as usize] {
            require!(false);
        }
        require!(false);
    }
    OneVsAll::Yes
}

#[rustfmt::skip]
pub fn big_mouth(cc: Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>) -> Option<f32> {
    if <crate::line_segment_sketch::concave_component::ConcaveComponent>::relative_bounding_box(cc).deleash().ymax() > 0.5f32 {
        require!(cc.deleash().strokes.deleash().first().unwrap().deleash().start.x > cc.deleash().strokes.deleash().first().unwrap().deleash().end.x);
    }
    Some(<crate::line_segment_sketch::concave_component::ConcaveComponent>::relative_bounding_box(cc).deleash().ymax())
}