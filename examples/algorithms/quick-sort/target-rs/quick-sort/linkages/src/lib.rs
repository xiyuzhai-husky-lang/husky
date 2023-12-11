use husky_core::*;
use ad_hoc_task_dependency::*;
use quick_sort::*;

#[rustfmt::skip]
linkage_impls![
    quick_sort::quick_sort_works_for_integers,
    quick_sort::quick_sort_works_for_strs,
];