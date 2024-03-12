#![feature(trait_upcasting)]
use husky_core::*;
use ad_hoc_task_dependency::{*, ugly::*};
use malamute::*;

#[rustfmt::skip]
linkage_impls![
    enum_u8_presenter_linkage_impl!(malamute::OneVsAll),
    enum_variant_unit_constructor_linkage_impl!(malamute::OneVsAll::Yes),
    enum_variant_discriminator_linkage_impl!(malamute::OneVsAll, malamute::OneVsAll::__Yes_discriminator),
    enum_variant_unit_constructor_linkage_impl!(malamute::OneVsAll::No),
    enum_variant_discriminator_linkage_impl!(malamute::OneVsAll, malamute::OneVsAll::__No_discriminator),
    enum_u8_presenter_linkage_impl!(malamute::OneVsAllResult),
    enum_variant_unit_constructor_linkage_impl!(malamute::OneVsAllResult::ConfidentYes),
    enum_variant_discriminator_linkage_impl!(malamute::OneVsAllResult, malamute::OneVsAllResult::__ConfidentYes_discriminator),
    enum_variant_unit_constructor_linkage_impl!(malamute::OneVsAllResult::ConfidentNo),
    enum_variant_discriminator_linkage_impl!(malamute::OneVsAllResult, malamute::OneVsAllResult::__ConfidentNo_discriminator),
    enum_variant_unit_constructor_linkage_impl!(malamute::OneVsAllResult::Unconfident),
    enum_variant_discriminator_linkage_impl!(malamute::OneVsAllResult, malamute::OneVsAllResult::__Unconfident_discriminator),
    fn_linkage_impl!(<malamute::OneVsAll as Default>::default),
    fn_linkage_impl!(<malamute::OneVsAll as Unveil<malamute::OneVsAllResult>>::unveil),
    unveil_linkage_impl!(<malamute::OneVsAll as Unveil<malamute::OneVsAllResult>>::unveil),
    enum_variant_tuple_constructor_linkage_impl!(husky_core::ops::ControlFlow::Continue),
    enum_variant_discriminator_linkage_impl!(husky_core::ops::ControlFlow<malamute::OneVsAll, ()>, husky_core::ops::ControlFlow::__Continue_discriminator),
    enum_variant_destructor_linkage_impl!(husky_core::ops::ControlFlow<malamute::OneVsAll, ()>, husky_core::ops::ControlFlow::__Continue_destructor, v0),
    enum_variant_field_linkage_impl!(husky_core::ops::ControlFlow<malamute::OneVsAll, ()>, husky_core::ops::ControlFlow::Continue, .0),
    enum_variant_tuple_constructor_linkage_impl!(husky_core::ops::ControlFlow::Break),
    enum_variant_discriminator_linkage_impl!(husky_core::ops::ControlFlow<malamute::OneVsAll, ()>, husky_core::ops::ControlFlow::__Break_discriminator),
    enum_variant_destructor_linkage_impl!(husky_core::ops::ControlFlow<malamute::OneVsAll, ()>, husky_core::ops::ControlFlow::__Break_destructor, v0),
    enum_variant_field_linkage_impl!(husky_core::ops::ControlFlow<malamute::OneVsAll, ()>, husky_core::ops::ControlFlow::Break, .0),
];