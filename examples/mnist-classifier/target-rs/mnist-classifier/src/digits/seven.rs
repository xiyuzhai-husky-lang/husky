use super::*;

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::val(ingredient_index = 38, return_leash)]
pub fn simple_seven_match() -> crate::fermi::FermiMatchResult {
    crate::fermi::fermi_match(major_concave_components(), vec![simple_leftdown_pattern])
}

#[rustfmt::skip]
pub fn simple_leftdown_pattern(cc: Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>) -> Option<f32> {
    let dp = cc.deleash().displacement();
    require!(dp.y < 0.0f32);
    Some(-dp.y)
}

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::val(ingredient_index = 39, return_leash)]
pub fn special_seven_match() -> crate::fermi::FermiMatchResult {
    crate::fermi::fermi_match(major_concave_components(), vec![leftupcc_pattern, leftdowncc_pattern])
}

#[rustfmt::skip]
pub fn leftupcc_pattern(cc: Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>) -> Option<f32> {
    let dp = cc.deleash().displacement();
    require!(dp.y < 0.0f32);
    require!(<crate::line_segment_sketch::concave_component::ConcaveComponent>::relative_bounding_box(cc).deleash().ymax() > 0.6f32);
    Some(cc.deleash().end().y)
}

#[rustfmt::skip]
pub fn leftdowncc_pattern(cc: Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>) -> Option<f32> {
    let dp = cc.deleash().displacement();
    require!(dp.y < 0.0f32);
    require!(<crate::line_segment_sketch::concave_component::ConcaveComponent>::relative_bounding_box(cc).deleash().ymin() < 0.3f32);
    let ang = cc.deleash().start_tangent().angle(true);
    require!(ang < 30.0f32);
    Some(ang)
}

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::val(ingredient_index = 40)]
pub fn is_seven() -> malamute::OneVsAll {
    require!(<crate::connected_component::ConnectedComponent>::max_hole_ilen(major_connected_component()) == 0.0f32);
    let simple_match_norm = <crate::fermi::FermiMatchResult>::norm(Leash(&simple_seven_match()));
    if simple_match_norm < 1.0f32 {
        require!(let Some(_) = simple_seven_match().matches[0 as usize]);
        let upper_excess = <crate::connected_component::ConnectedComponent>::upper_mass(major_connected_component()) - <crate::connected_component::ConnectedComponent>::lower_mass(major_connected_component());
        if upper_excess < 10.0f32 {
            let end_tangent = simple_seven_match().matches[0 as usize].unwrap().deleash().end_tangent();
            let a = end_tangent.y;
            require!(a < -7.0f32);
        }
        return OneVsAll::Yes;
    }
    if simple_match_norm < 4.0f32 {
        let upper_excess = <crate::connected_component::ConnectedComponent>::upper_mass(major_connected_component()) - <crate::connected_component::ConnectedComponent>::lower_mass(major_connected_component());
        require!(upper_excess > 10.0f32);
        return OneVsAll::Yes;
    }
    require!(let Some(_) = special_seven_match().matches[0 as usize]);
    let others = &special_seven_match().others;
    require!(false);
    OneVsAll::Yes
}