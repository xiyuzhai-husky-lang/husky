#![allow(warnings, non_snake_case)]
use ad_hoc_task_dependency::{ugly::*, *};
use husky_core::*;

ad_hoc_task_dependency::init_crate!();

#[ad_hoc_task_dependency::value_conversion]
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Class<Label> {
    Known(Label),
    Unknown,
}

#[ad_hoc_task_dependency::value_conversion]
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum OneVsAll {
    Yes,
    No,
}

#[ad_hoc_task_dependency::value_conversion]
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum OneVsAllResult {
    ConfidentYes,
    ConfidentNo,
    Unconfident,
}

impl Default for crate::OneVsAll {
    fn default() -> crate::OneVsAll {
        OneVsAll::No
    }
}

impl<Label> Unveil<crate::OneVsAll> for crate::Class<Label>
where
    Label: Copy,
{
    type RuntimeConstSymbols = (Label);

    type Output = ();

    fn unveil(
        one_vs_all: crate::OneVsAll,
        (label): &(Label),
    ) -> husky_core::ops::ControlFlow<crate::Class<Label>, ()> {
        match one_vs_all {
            crate::OneVsAll::Yes => {
                husky_core::ops::ControlFlow::Break(crate::Class::Known(*label))
            }
            crate::OneVsAll::No => husky_core::ops::ControlFlow::Continue(()),
        }
    }
}

impl Unveil<crate::OneVsAllResult> for crate::OneVsAll {
    type RuntimeConstSymbols = ();

    type Output = ();

    fn unveil(
        one_vs_all_result: crate::OneVsAllResult,
        (): &(),
    ) -> husky_core::ops::ControlFlow<crate::OneVsAll, ()> {
        match one_vs_all_result {
            crate::OneVsAllResult::ConfidentYes => {
                husky_core::ops::ControlFlow::Break(OneVsAll::Yes)
            }
            crate::OneVsAllResult::ConfidentNo => husky_core::ops::ControlFlow::Break(OneVsAll::No),
            crate::OneVsAllResult::Unconfident => husky_core::ops::ControlFlow::Continue(()),
        }
    }
}

pub struct narrow_down<Label>(std::marker::PhantomData<Label>);

impl<Label> __IsGnItem for narrow_down<Label> {
    type Pedestal = __Pedestal;

    fn generic_pedestal(generic_pedestal: __Pedestal) -> __Pedestal {
        __Pedestal::Generic
    }

    type ValueAtGenericPedestal = ();

    fn train(
        val_argument_reprs: &[__ValArgumentReprInterface],
    ) -> Result<Self::ValueAtGenericPedestal, ()> {
        let __ValArgumentReprInterface::Variadic(ref features) = val_argument_reprs[0] else {
            unreachable!()
        };
        let __ValArgumentReprInterface::Keyed(skip) = val_argument_reprs[1] else {
            unreachable!()
        };
        let skip = match skip {
            Some(skip) => __eval_val_repr(skip),
            None => todo!(),
        };
        println!("val_argument_reprs.len() = {}", val_argument_reprs.len());
        todo!()
    }

    fn eval(
        val_argument_reprs: &[__ValArgumentReprInterface],
        value_at_generic_pedestal: &Self::ValueAtGenericPedestal,
    ) -> __ValueResult {
        todo!()
    }
}
