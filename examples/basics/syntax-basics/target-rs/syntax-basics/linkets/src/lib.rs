#![feature(trait_upcasting)]
use husky_core::*;
use ad_hoc_devsoul_dependency::{*, ugly::*};
use syntax_basics::*;

#[rustfmt::skip]
linket_impls![
    enum_variant_constructor_linket_impl!(syntax_basics::defn::major_item::ty::enum_ty::A, syntax_basics::defn::major_item::ty::enum_ty::A::UnitVariant),
    enum_variant_discriminator_linket_impl!(syntax_basics::defn::major_item::ty::enum_ty::A, syntax_basics::defn::major_item::ty::enum_ty::A::UnitVariant),
    enum_variant_constructor_linket_impl!(syntax_basics::defn::major_item::ty::enum_ty::A, syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithOneField, (x)),
    enum_variant_discriminator_linket_impl!(syntax_basics::defn::major_item::ty::enum_ty::A, syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithOneField, {}),
    enum_variant_destructor_linket_impl!(syntax_basics::defn::major_item::ty::enum_ty::A, syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithOneField, {x}),
    enum_variant_field_linket_impl!(syntax_basics::defn::major_item::ty::enum_ty::A, syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithOneField, {x}),
    enum_variant_constructor_linket_impl!(syntax_basics::defn::major_item::ty::enum_ty::A, syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithTwoFields, (x, y)),
    enum_variant_discriminator_linket_impl!(syntax_basics::defn::major_item::ty::enum_ty::A, syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithTwoFields, {}),
    enum_variant_destructor_linket_impl!(syntax_basics::defn::major_item::ty::enum_ty::A, syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithTwoFields, {x, y}),
    enum_variant_field_linket_impl!(syntax_basics::defn::major_item::ty::enum_ty::A, syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithTwoFields, {x}),
    enum_variant_field_linket_impl!(syntax_basics::defn::major_item::ty::enum_ty::A, syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithTwoFields, {y}),
    enum_variant_constructor_linket_impl!(syntax_basics::defn::major_item::ty::enum_ty::A, syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithOneField, (v0)),
    enum_variant_discriminator_linket_impl!(syntax_basics::defn::major_item::ty::enum_ty::A, syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithOneField, ()),
    enum_variant_destructor_linket_impl!(syntax_basics::defn::major_item::ty::enum_ty::A, syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithOneField, (v0)),
    enum_variant_field_linket_impl!(syntax_basics::defn::major_item::ty::enum_ty::A, syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithOneField, (v0)),
    enum_variant_constructor_linket_impl!(syntax_basics::defn::major_item::ty::enum_ty::A, syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithTwoFields, (v0, v1)),
    enum_variant_discriminator_linket_impl!(syntax_basics::defn::major_item::ty::enum_ty::A, syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithTwoFields, ()),
    enum_variant_destructor_linket_impl!(syntax_basics::defn::major_item::ty::enum_ty::A, syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithTwoFields, (v0, v1)),
    enum_variant_field_linket_impl!(syntax_basics::defn::major_item::ty::enum_ty::A, syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithTwoFields, (v0)),
    enum_variant_field_linket_impl!(syntax_basics::defn::major_item::ty::enum_ty::A, syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithTwoFields, (v1)),
    fn_linket_impl!(syntax_basics::expr::nested),
    fn_linket_impl!(syntax_basics::expr::closure_inline),
    fn_linket_impl!(syntax_basics::expr::closure_nested),
];