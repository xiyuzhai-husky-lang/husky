use crate::*;
use husky_value_protocol::presentation::ValuePresentation;
use serde::{Deserialize, Serialize};
use std::ops::{FromResidual, Residual, Try};

/// machine control flows
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum VmControlFlow<C, B, E> {
    Continue(C),
    LoopContinue,
    LoopExit(B),
    Return(B),
    Throw(E),
}

impl<C, B, E> VmControlFlow<C, B, E> {
    pub fn map<D>(self, f: impl FnOnce(C) -> D) -> VmControlFlow<D, B, E> {
        match self {
            VmControlFlow::Continue(c) => VmControlFlow::Continue(f(c)),
            VmControlFlow::LoopContinue => VmControlFlow::LoopContinue,
            VmControlFlow::LoopExit(b) => VmControlFlow::LoopExit(b),
            VmControlFlow::Return(b) => VmControlFlow::Return(b),
            VmControlFlow::Throw(e) => VmControlFlow::Throw(e),
        }
    }
}

pub type ValuePresentationVmControlFlow =
    VmControlFlow<ValuePresentation, ValuePresentation, ValuePresentation>;

impl<C, B, E> Residual<C> for VmControlFlow<Infallible, B, E> {
    type TryType = VmControlFlow<C, B, E>;
}

impl<C, B, E> Try for VmControlFlow<C, B, E> {
    type Output = C;

    type Residual = VmControlFlow<Infallible, B, E>;

    fn from_output(_output: Self::Output) -> Self {
        todo!()
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            VmControlFlow::Continue(c) => std::ops::ControlFlow::Continue(c),
            VmControlFlow::LoopContinue => {
                std::ops::ControlFlow::Break(VmControlFlow::LoopContinue)
            }
            VmControlFlow::LoopExit(b) => std::ops::ControlFlow::Break(VmControlFlow::LoopExit(b)),
            VmControlFlow::Return(b) => std::ops::ControlFlow::Break(VmControlFlow::Return(b)),
            VmControlFlow::Throw(e) => std::ops::ControlFlow::Break(VmControlFlow::Throw(e)),
        }
    }
}

impl<C, B, E> FromResidual<VmControlFlow<Infallible, B, E>> for VmControlFlow<C, B, E> {
    fn from_residual(residual: VmControlFlow<Infallible, B, E>) -> Self {
        match residual {
            VmControlFlow::Continue(_) => unreachable!(),
            VmControlFlow::LoopContinue => VmControlFlow::LoopContinue,
            VmControlFlow::LoopExit(b) => VmControlFlow::LoopExit(b),
            VmControlFlow::Return(b) => VmControlFlow::Return(b),
            VmControlFlow::Throw(e) => VmControlFlow::Throw(e),
        }
    }
}

impl<C, B, E> FromResidual<Result<Infallible, E>> for VmControlFlow<C, B, E> {
    fn from_residual(residual: Result<Infallible, E>) -> Self {
        match residual {
            Ok(_) => unreachable!(),
            Err(e) => VmControlFlow::Throw(e),
        }
    }
}

impl<C1, C2: FromIterator<C1>, B, E> std::iter::FromIterator<VmControlFlow<C1, B, E>>
    for VmControlFlow<C2, B, E>
{
    fn from_iter<T: IntoIterator<Item = VmControlFlow<C1, B, E>>>(iter: T) -> Self {
        match iter
            .into_iter()
            .map(|item| match item.branch() {
                std::ops::ControlFlow::Continue(c1) => Ok(c1),
                std::ops::ControlFlow::Break(residual) => Err(residual),
            })
            .collect::<Result<C2, VmControlFlow<Infallible, B, E>>>()
        {
            Ok(c2) => VmControlFlow::Continue(c2),
            Err(residual) => VmControlFlow::from_residual(residual),
        }
    }
}

impl<ThawedValue, E> VmControlFlow<ThawedValue, ThawedValue, E>
where
    ThawedValue: IsThawedValue,
    E: Clone,
{
    pub fn freeze(
        &self,
    ) -> VmControlFlow<
        <ThawedValue::Value as IsValue>::FrozenValue,
        <ThawedValue::Value as IsValue>::FrozenValue,
        E,
    > {
        match self {
            VmControlFlow::Continue(v) => VmControlFlow::Continue(v.freeze()),
            VmControlFlow::LoopContinue => todo!(),
            VmControlFlow::LoopExit(_) => todo!(),
            VmControlFlow::Return(_) => todo!(),
            VmControlFlow::Throw(e) => VmControlFlow::Throw(e.clone()),
        }
    }
}

impl<FrozenValue, E> VmControlFlow<FrozenValue, FrozenValue, E>
where
    E: Clone + Send + Sync + 'static,
    FrozenValue: IsFrozenValue,
{
    pub fn present(
        &self,
        value_presenter_cache: &mut ValuePresenterCache,
        value_presentation_synchrotron: &mut ValuePresentationSynchrotron,
    ) -> ValuePresentationVmControlFlow
    where
        E: std::fmt::Debug + Serialize,
    {
        match self {
            VmControlFlow::Continue(value) => VmControlFlow::Continue(
                value.present(value_presenter_cache, value_presentation_synchrotron),
            ),
            VmControlFlow::LoopContinue => VmControlFlow::LoopContinue,
            VmControlFlow::LoopExit(value) => VmControlFlow::LoopExit(
                value.present(value_presenter_cache, value_presentation_synchrotron),
            ),
            VmControlFlow::Return(value) => VmControlFlow::Return(
                value.present(value_presenter_cache, value_presentation_synchrotron),
            ),
            VmControlFlow::Throw(e) => {
                VmControlFlow::Throw(ValuePresentation::AdHoc(format! {"{e:?}"}))
            }
        }
    }
}
