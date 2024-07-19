#![feature(trait_upcasting)]
use husky_core::*;
use ad_hoc_devsoul_dependency::{*, ugly::*};
use syntax_errors::*;

#[rustfmt::skip]
linket_impls![
    fn_linket_impl!(syntax_errors::ast::A::__constructor),
];