#![feature(trait_upcasting)]
use husky_core::*;
use ad_hoc_devsoul_dependency::{*, ugly::*};

#[rustfmt::skip]
linket_impls![
    enum_index_presenter_linket_impl!(malamute::OneVsAll),
    enum_variant_constructor_linket_impl!(malamute::OneVsAll, malamute::OneVsAll::Yes),
    enum_variant_discriminator_linket_impl!(malamute::OneVsAll, malamute::OneVsAll::Yes),
    enum_variant_constructor_linket_impl!(malamute::OneVsAll, malamute::OneVsAll::No),
    enum_variant_discriminator_linket_impl!(malamute::OneVsAll, malamute::OneVsAll::No),
    enum_index_presenter_linket_impl!(malamute::OneVsAllResult),
    enum_variant_constructor_linket_impl!(malamute::OneVsAllResult, malamute::OneVsAllResult::ConfidentYes),
    enum_variant_discriminator_linket_impl!(malamute::OneVsAllResult, malamute::OneVsAllResult::ConfidentYes),
    enum_variant_constructor_linket_impl!(malamute::OneVsAllResult, malamute::OneVsAllResult::ConfidentNo),
    enum_variant_discriminator_linket_impl!(malamute::OneVsAllResult, malamute::OneVsAllResult::ConfidentNo),
    enum_variant_constructor_linket_impl!(malamute::OneVsAllResult, malamute::OneVsAllResult::Unconfident),
    enum_variant_discriminator_linket_impl!(malamute::OneVsAllResult, malamute::OneVsAllResult::Unconfident),
    fn_linket_impl!(<malamute::OneVsAll as Default>::default),
    fn_linket_impl!(<malamute::OneVsAll as Unveil<malamute::OneVsAllResult>>::unveil),
    unveil_fn_linket_impl!(<malamute::OneVsAll as Unveil<malamute::OneVsAllResult>>::unveil),
    enum_variant_constructor_linket_impl!(husky_core::ops::ControlFlow<malamute::OneVsAll, ()>, husky_core::ops::ControlFlow::Continue, (v0)),
    enum_variant_discriminator_linket_impl!(husky_core::ops::ControlFlow<malamute::OneVsAll, ()>, husky_core::ops::ControlFlow::Continue, ()),
    enum_variant_destructor_linket_impl!(husky_core::ops::ControlFlow<malamute::OneVsAll, ()>, husky_core::ops::ControlFlow::Continue, (v0)),
    enum_variant_field_linket_impl!(husky_core::ops::ControlFlow<malamute::OneVsAll, ()>, husky_core::ops::ControlFlow::Continue, (copyable v0)),
    enum_variant_constructor_linket_impl!(husky_core::ops::ControlFlow<malamute::OneVsAll, ()>, husky_core::ops::ControlFlow::Break, (v0)),
    enum_variant_discriminator_linket_impl!(husky_core::ops::ControlFlow<malamute::OneVsAll, ()>, husky_core::ops::ControlFlow::Break, ()),
    enum_variant_destructor_linket_impl!(husky_core::ops::ControlFlow<malamute::OneVsAll, ()>, husky_core::ops::ControlFlow::Break, (v0)),
    enum_variant_field_linket_impl!(husky_core::ops::ControlFlow<malamute::OneVsAll, ()>, husky_core::ops::ControlFlow::Break, (other v0)),
];