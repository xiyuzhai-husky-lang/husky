#![feature(trait_upcasting)]
use husky_core::*;
use ad_hoc_devsoul_dependency::{*, ugly::*};

#[rustfmt::skip]
linket_impls![
    fn_linket_impl!(quick_sort::quick_sort_works_for_integers),
    fn_linket_impl!(quick_sort::quick_sort_works_for_strs),
    fn_linket_impl!(|v: Vec<i32>|v),
    fn_linket_impl!(|v: Vec<&'static str>|v),
];