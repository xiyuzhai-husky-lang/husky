#![feature(trait_upcasting)]
use husky_core::*;
use ad_hoc_task_dependency::{*, ugly::*};
use syntax_basics::*;

#[rustfmt::skip]
linkage_impls![
    fn_linkage_impl!(syntax_basics::expr::nested),
    fn_linkage_impl!(syntax_basics::expr::closure_inline),
    fn_linkage_impl!(syntax_basics::expr::closure_nested),
];