use super::*;

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