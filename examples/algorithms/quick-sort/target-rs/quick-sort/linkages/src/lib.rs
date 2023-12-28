use husky_core::*;
use ad_hoc_task_dependency::{*, ugly::*};
use quick_sort::*;

#[rustfmt::skip]
linkage_impls![
    fn_linkage_impl!(quick_sort::quick_sort_works_for_integers),
    fn_linkage_impl!(quick_sort::quick_sort_works_for_strs),
    fn_linkage_impl!(|v: Vec<i32>|v),
    fn_linkage_impl!(|v: Vec<&'static str>|v),
];