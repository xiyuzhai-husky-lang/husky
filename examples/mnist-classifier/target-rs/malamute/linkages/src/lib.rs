use husky_core::*;
use ad_hoc_task_dependency::{*, ugly::*};
use malamute::*;

#[rustfmt::skip]
linkage_impls![
    <malamute::OneVsAll as Default>::default,
    <malamute::Class as Unveil>::unveil,
    <malamute::OneVsAll as Unveil>::unveil,
];