#![feature(trait_upcasting)]
use husky_core::*;
use ad_hoc_devsoul_dependency::{*, ugly::*};

#[rustfmt::skip]
linket_impls![
    enum_variant_constructor_linket_impl!(syntax_basics::defn::major_item::ty::enum_ty::A, syntax_basics::defn::major_item::ty::enum_ty::A::UnitVariant),
    enum_variant_discriminator_linket_impl!(syntax_basics::defn::major_item::ty::enum_ty::A, syntax_basics::defn::major_item::ty::enum_ty::A::UnitVariant),
    enum_variant_constructor_linket_impl!(syntax_basics::defn::major_item::ty::enum_ty::A, syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithOneField, (x)),
    enum_variant_discriminator_linket_impl!(syntax_basics::defn::major_item::ty::enum_ty::A, syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithOneField, {}),
    enum_variant_destructor_linket_impl!(syntax_basics::defn::major_item::ty::enum_ty::A, syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithOneField, {x}),
    enum_variant_field_linket_impl!(syntax_basics::defn::major_item::ty::enum_ty::A, syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithOneField, {copyable x}),
    enum_variant_constructor_linket_impl!(syntax_basics::defn::major_item::ty::enum_ty::A, syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithTwoFields, (x, y)),
    enum_variant_discriminator_linket_impl!(syntax_basics::defn::major_item::ty::enum_ty::A, syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithTwoFields, {}),
    enum_variant_destructor_linket_impl!(syntax_basics::defn::major_item::ty::enum_ty::A, syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithTwoFields, {x, y}),
    enum_variant_field_linket_impl!(syntax_basics::defn::major_item::ty::enum_ty::A, syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithTwoFields, {copyable x}),
    enum_variant_field_linket_impl!(syntax_basics::defn::major_item::ty::enum_ty::A, syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithTwoFields, {copyable y}),
    enum_variant_constructor_linket_impl!(syntax_basics::defn::major_item::ty::enum_ty::A, syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithOneField, (v0)),
    enum_variant_discriminator_linket_impl!(syntax_basics::defn::major_item::ty::enum_ty::A, syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithOneField, ()),
    enum_variant_destructor_linket_impl!(syntax_basics::defn::major_item::ty::enum_ty::A, syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithOneField, (v0)),
    enum_variant_field_linket_impl!(syntax_basics::defn::major_item::ty::enum_ty::A, syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithOneField, (copyable v0)),
    enum_variant_constructor_linket_impl!(syntax_basics::defn::major_item::ty::enum_ty::A, syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithTwoFields, (v0, v1)),
    enum_variant_discriminator_linket_impl!(syntax_basics::defn::major_item::ty::enum_ty::A, syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithTwoFields, ()),
    enum_variant_destructor_linket_impl!(syntax_basics::defn::major_item::ty::enum_ty::A, syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithTwoFields, (v0, v1)),
    enum_variant_field_linket_impl!(syntax_basics::defn::major_item::ty::enum_ty::A, syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithTwoFields, (copyable v0)),
    enum_variant_field_linket_impl!(syntax_basics::defn::major_item::ty::enum_ty::A, syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithTwoFields, (copyable v1)),
    fn_linket_impl!(syntax_basics::expr::nested),
    fn_linket_impl!(syntax_basics::expr::closure_inline),
    fn_linket_impl!(syntax_basics::expr::closure_nested),
];