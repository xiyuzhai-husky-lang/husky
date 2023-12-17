#![allow(warnings, non_snake_case)]
use husky_core::*;
use ad_hoc_task_dependency::{*, ugly::*};

ad_hoc_task_dependency::init_crate!();

#[rustfmt::skip]
#[ad_hoc_task_dependency::value_conversion]
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Class<Label> {
    Known(Label),
    Unknown,
}

#[rustfmt::skip]
#[ad_hoc_task_dependency::value_conversion]
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum OneVsAll {
    Yes,
    No,
}

#[rustfmt::skip]
#[ad_hoc_task_dependency::value_conversion]
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum OneVsAllResult {
    ConfidentYes,
    ConfidentNo,
    Unconfident,
}

#[rustfmt::skip]
impl Default for crate::OneVsAll {
    fn default() -> crate::OneVsAll {
        OneVsAll::No
    }
}

#[rustfmt::skip]
impl <Label, >Unveil<crate::OneVsAll> for crate::Class<Label> {
    
    type Output = ();
}

#[rustfmt::skip]
impl Unveil<crate::OneVsAllResult> for crate::OneVsAll {
    
    type Output = ();
}