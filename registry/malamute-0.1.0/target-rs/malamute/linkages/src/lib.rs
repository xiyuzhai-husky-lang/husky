use husky_core::*;
use ad_hoc_task_dependency::*;
use malamute::*;

#[rustfmt::skip]
linkages![
    OneVsAll,
    OneVsAllResult,
    <OneVsAll as Default>::default,
    <Class as Unveil>::Output,
    <OneVsAll as Unveil>::Output,
];