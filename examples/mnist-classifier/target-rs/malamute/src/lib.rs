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
impl <Label, >Unveil<crate::OneVsAll> for crate::Class<Label>
where Label: Copy {
    type RuntimeConstSymbols = (Label);

    
    type Output = ();

    fn unveil(one_vs_all: crate::OneVsAll, (label): &(Label)) -> husky_core::ops::ControlFlow<crate::Class<Label>, ()> {
        match one_vs_all{
            crate::OneVsAll::Yes => {
                husky_core::ops::ControlFlow::Break(crate::Class::Known(*label))
            }
            crate::OneVsAll::No => {
                husky_core::ops::ControlFlow::Continue(())
            }
        }
    }
}

#[rustfmt::skip]
impl Unveil<crate::OneVsAllResult> for crate::OneVsAll
where {
    type RuntimeConstSymbols = ();

    
    type Output = ();

    fn unveil(one_vs_all_result: crate::OneVsAllResult, (): &()) -> husky_core::ops::ControlFlow<crate::OneVsAll, ()> {
        match one_vs_all_result{
            crate::OneVsAllResult::ConfidentYes => {
                husky_core::ops::ControlFlow::Break(OneVsAll::Yes)
            }
            crate::OneVsAllResult::ConfidentNo => {
                husky_core::ops::ControlFlow::Break(OneVsAll::No)
            }
            crate::OneVsAllResult::Unconfident => {
                husky_core::ops::ControlFlow::Continue(())
            }
        }
    }
}