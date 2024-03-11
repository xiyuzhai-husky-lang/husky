#![feature(trait_upcasting)]
use husky_core::*;
use ad_hoc_task_dependency::{*, ugly::*};
use malamute::*;

#[rustfmt::skip]
linkage_impls![
    enum_u8_presenter_linkage_impl!(malamute::OneVsAll),
    fn_linkage_impl!(malamute::OneVsAll::Yes),
    fn_linkage_impl!(malamute::OneVsAll::__Yes_discriminator),
    fn_linkage_impl!(malamute::OneVsAll::No),
    fn_linkage_impl!(malamute::OneVsAll::__No_discriminator),
    enum_u8_presenter_linkage_impl!(malamute::OneVsAllResult),
    fn_linkage_impl!(malamute::OneVsAllResult::ConfidentYes),
    fn_linkage_impl!(malamute::OneVsAllResult::__ConfidentYes_discriminator),
    fn_linkage_impl!(malamute::OneVsAllResult::ConfidentNo),
    fn_linkage_impl!(malamute::OneVsAllResult::__ConfidentNo_discriminator),
    fn_linkage_impl!(malamute::OneVsAllResult::Unconfident),
    fn_linkage_impl!(malamute::OneVsAllResult::__Unconfident_discriminator),
    fn_linkage_impl!(<malamute::OneVsAll as Default>::default),
    fn_linkage_impl!(<malamute::OneVsAll as Unveil<malamute::OneVsAllResult>>::unveil),
    unveil_linkage_impl!(<malamute::OneVsAll as Unveil<malamute::OneVsAllResult>>::unveil),
    fn_linkage_impl!(husky_core::ops::ControlFlow::Continue::<malamute::OneVsAll, ()>),
    fn_linkage_impl!(husky_core::ops::ControlFlow::__Continue_discriminator::<malamute::OneVsAll, ()>),
    destructor_linkage_impl!(husky_core::ops::ControlFlow::__Continue_destructor::<malamute::OneVsAll, ()>),
    enum_field_linkage_impl!(husky_core::ops::ControlFlow<malamute::OneVsAll, ()>, husky_core::ops::ControlFlow::Continue, .0),
    fn_linkage_impl!(husky_core::ops::ControlFlow::Break::<malamute::OneVsAll, ()>),
    fn_linkage_impl!(husky_core::ops::ControlFlow::__Break_discriminator::<malamute::OneVsAll, ()>),
    destructor_linkage_impl!(husky_core::ops::ControlFlow::__Break_destructor::<malamute::OneVsAll, ()>),
    enum_field_linkage_impl!(husky_core::ops::ControlFlow<malamute::OneVsAll, ()>, husky_core::ops::ControlFlow::Break, .0),
];