#![feature(trait_upcasting)]
use husky_core::*;
use ad_hoc_task_dependency::{*, ugly::*};
use malamute::*;

#[rustfmt::skip]
linkage_impls![
    enum_index_presenter_linkage_impl!(malamute::OneVsAll),
    enum_variant_constructor_linkage_impl!(malamute::OneVsAll, malamute::OneVsAll::Yes),
    enum_variant_discriminator_linkage_impl!(malamute::OneVsAll, malamute::OneVsAll::Yes),
    enum_variant_constructor_linkage_impl!(malamute::OneVsAll, malamute::OneVsAll::No),
    enum_variant_discriminator_linkage_impl!(malamute::OneVsAll, malamute::OneVsAll::No),
    enum_index_presenter_linkage_impl!(malamute::OneVsAllResult),
    enum_variant_constructor_linkage_impl!(malamute::OneVsAllResult, malamute::OneVsAllResult::ConfidentYes),
    enum_variant_discriminator_linkage_impl!(malamute::OneVsAllResult, malamute::OneVsAllResult::ConfidentYes),
    enum_variant_constructor_linkage_impl!(malamute::OneVsAllResult, malamute::OneVsAllResult::ConfidentNo),
    enum_variant_discriminator_linkage_impl!(malamute::OneVsAllResult, malamute::OneVsAllResult::ConfidentNo),
    enum_variant_constructor_linkage_impl!(malamute::OneVsAllResult, malamute::OneVsAllResult::Unconfident),
    enum_variant_discriminator_linkage_impl!(malamute::OneVsAllResult, malamute::OneVsAllResult::Unconfident),
    fn_linkage_impl!(<malamute::OneVsAll as Default>::default),
    fn_linkage_impl!(<malamute::OneVsAll as Unveil<malamute::OneVsAllResult>>::unveil),
    unveil_linkage_impl!(<malamute::OneVsAll as Unveil<malamute::OneVsAllResult>>::unveil),
    enum_variant_constructor_linkage_impl!(husky_core::ops::ControlFlow<malamute::OneVsAll, ()>, husky_core::ops::ControlFlow::Continue, (v0)),
    enum_variant_discriminator_linkage_impl!(husky_core::ops::ControlFlow<malamute::OneVsAll, ()>, husky_core::ops::ControlFlow::Continue, ()),
    enum_variant_destructor_linkage_impl!(husky_core::ops::ControlFlow<malamute::OneVsAll, ()>, husky_core::ops::ControlFlow::Continue, (v0)),
    enum_variant_field_linkage_impl!(husky_core::ops::ControlFlow<malamute::OneVsAll, ()>, husky_core::ops::ControlFlow::Continue, (v0)),
    enum_variant_constructor_linkage_impl!(husky_core::ops::ControlFlow<malamute::OneVsAll, ()>, husky_core::ops::ControlFlow::Break, (v0)),
    enum_variant_discriminator_linkage_impl!(husky_core::ops::ControlFlow<malamute::OneVsAll, ()>, husky_core::ops::ControlFlow::Break, ()),
    enum_variant_destructor_linkage_impl!(husky_core::ops::ControlFlow<malamute::OneVsAll, ()>, husky_core::ops::ControlFlow::Break, (v0)),
    enum_variant_field_linkage_impl!(husky_core::ops::ControlFlow<malamute::OneVsAll, ()>, husky_core::ops::ControlFlow::Break, (v0)),
];