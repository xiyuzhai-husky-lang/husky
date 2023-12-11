use ad_hoc_task_dependency::*;
use husky_core::*;
use malamute::*;

#[rustfmt::skip]
linkages![
    OneVsAll,
    OneVsAllResult,
    <OneVsAll as Default>::default,
    <Class as Unveil>::Output,
    <OneVsAll as Unveil>::Output,
];
