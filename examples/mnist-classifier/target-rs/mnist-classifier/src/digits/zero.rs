use super::*;

#[rustfmt::skip]
#[ad_hoc_task_dependency::val_item(ingredient_index = 24, return_ref)]
pub fn open_one_match() -> crate::fermi::FermiMatchResult {
    crate::fermi::fermi_match(major_concave_components(), &vec![almost_closed])
}

#[rustfmt::skip]
pub fn almost_closed(cc: Leash<crate::line_segment_sketch::concave_component::ConcaveComponent>) -> Option<f32> {
    require!(cc.angle_change() + 0.0f32 < -140.0f32);
    Some(-cc.angle_change() + 0.0f32)
}

#[rustfmt::skip]
#[ad_hoc_task_dependency::val_item(ingredient_index = 25, lazy)]
pub fn is_zero() -> malamute::OneVsAll {}