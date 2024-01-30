#![feature(trait_upcasting)]
use husky_core::*;
use ad_hoc_task_dependency::{*, ugly::*};
use malamute::*;

#[rustfmt::skip]
linkage_impls![
    enum_u8_presenter_linkage_impl!(malamute::OneVsAll),
    enum_u8_presenter_linkage_impl!(malamute::OneVsAllResult),
    fn_linkage_impl!(<malamute::OneVsAll as Default>::default),
    fn_linkage_impl!(<malamute::OneVsAll as Unveil<malamute::OneVsAllResult>>::unveil),
    unveil_fn_linkage_impl!(<malamute::OneVsAll as Unveil<malamute::OneVsAllResult>>::unveil),
    fn_linkage_impl!(husky_core::ops::ControlFlow::Break::<malamute::OneVsAll, ()>),
    fn_linkage_impl!(husky_core::ops::ControlFlow::Continue::<malamute::OneVsAll, ()>),
];