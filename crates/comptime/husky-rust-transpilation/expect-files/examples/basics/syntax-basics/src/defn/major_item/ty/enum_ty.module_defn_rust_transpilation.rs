use super::*;

#[allow(non_upper_case_globals)]
pub static mut __A__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

#[rustfmt::skip]
#[ad_hoc_devsoul_dependency::value_conversion]
#[derive(Debug, Clone, PartialEq)]
pub enum A {
    UnitVariant,
    PropsVariantWithOneField{x: i32},
    PropsVariantWithTwoFields{x: i32, y: f32},
    TupleVariantWithOneField(i32),
    TupleVariantWithTwoFields(i32, f32),
}