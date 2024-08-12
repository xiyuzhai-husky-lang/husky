use crate::exception::IsException;
use crate::*;
use husky_value_protocol::presentation::{
    synchrotron::ValuePresentationSynchrotron, ValuePresentation, ValuePresenterCache,
};
use serde::{Deserialize, Serialize};
use std::{
    ops::{FromResidual, Try},
    panic::panic_any,
};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum KiControlFlow<C, B, E> {
    Continue(C),
    LoopContinue,
    LoopExit(B),
    Return(B),
    Undefined,
    Throw(E),
}

impl<C, B, E> From<Result<C, E>> for KiControlFlow<C, B, E> {
    fn from(result: Result<C, E>) -> Self {
        match result {
            Ok(c) => KiControlFlow::Continue(c),
            Err(e) => KiControlFlow::Throw(e),
        }
    }
}

pub type ValuePresentationKiControlFlow =
    KiControlFlow<ValuePresentation, ValuePresentation, ValuePresentation>;

impl<C, B, E> std::ops::Residual<C> for KiControlFlow<Infallible, B, E> {
    type TryType = KiControlFlow<C, B, E>;
}

impl<C, B, E> std::ops::Try for KiControlFlow<C, B, E> {
    type Output = C;

    type Residual = KiControlFlow<Infallible, B, E>;

    fn from_output(_output: Self::Output) -> Self {
        todo!()
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            KiControlFlow::Continue(c) => std::ops::ControlFlow::Continue(c),
            KiControlFlow::LoopContinue => {
                std::ops::ControlFlow::Break(KiControlFlow::LoopContinue)
            }
            KiControlFlow::LoopExit(b) => std::ops::ControlFlow::Break(KiControlFlow::LoopExit(b)),
            KiControlFlow::Return(b) => std::ops::ControlFlow::Break(KiControlFlow::Return(b)),
            KiControlFlow::Undefined => std::ops::ControlFlow::Break(KiControlFlow::Undefined),
            KiControlFlow::Throw(e) => std::ops::ControlFlow::Break(KiControlFlow::Throw(e)),
        }
    }
}

impl<C, B1, B2, E> FromResidual<KiControlFlow<Infallible, B1, E>> for KiControlFlow<C, B2, E>
where
    B2: From<B1>,
{
    fn from_residual(residual: KiControlFlow<Infallible, B1, E>) -> Self {
        match residual {
            KiControlFlow::Continue(_) => unreachable!(),
            KiControlFlow::LoopContinue => KiControlFlow::LoopContinue,
            KiControlFlow::LoopExit(b) => KiControlFlow::LoopExit(b.into()),
            KiControlFlow::Return(b) => KiControlFlow::Return(b.into()),
            KiControlFlow::Undefined => KiControlFlow::Undefined,
            KiControlFlow::Throw(e) => KiControlFlow::Throw(e),
        }
    }
}

impl<C, B, E> FromResidual<Result<Infallible, E>> for KiControlFlow<C, B, E> {
    fn from_residual(residual: Result<Infallible, E>) -> Self {
        match residual {
            Ok(_) => unreachable!(),
            Err(e) => KiControlFlow::Throw(e),
        }
    }
}

impl<C1, C2: FromIterator<C1>, B, E> std::iter::FromIterator<KiControlFlow<C1, B, E>>
    for KiControlFlow<C2, B, E>
{
    fn from_iter<T: IntoIterator<Item = KiControlFlow<C1, B, E>>>(iter: T) -> Self {
        match iter
            .into_iter()
            .map(|item| match item.branch() {
                std::ops::ControlFlow::Continue(c1) => Ok(c1),
                std::ops::ControlFlow::Break(residual) => Err(residual),
            })
            .collect::<Result<C2, KiControlFlow<Infallible, B, E>>>()
        {
            Ok(c2) => KiControlFlow::Continue(c2),
            Err(residual) => KiControlFlow::from_residual(residual),
        }
    }
}

impl<Value, E> KiControlFlow<Value, Value, E>
where
    E: Clone + Send + Sync + 'static,
{
    pub unsafe fn share_unchecked(&self) -> Self
    where
        Value: IsValue,
    {
        let slf: &'static Self = std::mem::transmute(self);
        slf.share()
    }

    fn share(&'static self) -> Self
    where
        Value: IsValue,
    {
        match self {
            KiControlFlow::Continue(value) => KiControlFlow::Continue(value.share()),
            KiControlFlow::LoopContinue => todo!(),
            KiControlFlow::LoopExit(_) => todo!(),
            KiControlFlow::Return(_) => todo!(),
            KiControlFlow::Undefined => todo!(),
            KiControlFlow::Throw(e) => KiControlFlow::Throw(e.clone()),
        }
    }

    pub fn unwrap(self) -> Value {
        match self {
            KiControlFlow::Continue(v) => v,
            KiControlFlow::Throw(e) => panic_any(e),
            KiControlFlow::LoopContinue
            | KiControlFlow::LoopExit(_)
            | KiControlFlow::Return(_)
            | KiControlFlow::Undefined => unreachable!(),
        }
    }

    pub fn present(
        &self,
        value_presenter_cache: &mut ValuePresenterCache,
        value_presentation_synchrotron: &mut ValuePresentationSynchrotron,
    ) -> ValuePresentationKiControlFlow
    where
        Value: IsValue,
        E: std::fmt::Debug + Serialize,
    {
        match self {
            KiControlFlow::Continue(value) => KiControlFlow::Continue(
                value.present(value_presenter_cache, value_presentation_synchrotron),
            ),
            KiControlFlow::LoopContinue => KiControlFlow::LoopContinue,
            KiControlFlow::LoopExit(value) => KiControlFlow::LoopExit(
                value.present(value_presenter_cache, value_presentation_synchrotron),
            ),
            KiControlFlow::Return(value) => KiControlFlow::Return(
                value.present(value_presenter_cache, value_presentation_synchrotron),
            ),
            KiControlFlow::Undefined => KiControlFlow::Undefined,
            KiControlFlow::Throw(e) => {
                KiControlFlow::Throw(ValuePresentation::AdHoc(format! {"{e:?}"}))
            }
        }
    }
}

impl<C1, B, E> KiControlFlow<C1, B, E> {
    pub fn map<C2>(self, f: impl FnOnce(C1) -> C2) -> KiControlFlow<C2, B, E> {
        match self {
            KiControlFlow::Continue(c1) => KiControlFlow::Continue(f(c1)),
            KiControlFlow::LoopContinue => KiControlFlow::LoopContinue,
            KiControlFlow::LoopExit(b) => KiControlFlow::LoopExit(b),
            KiControlFlow::Return(b) => KiControlFlow::Return(b),
            KiControlFlow::Undefined => KiControlFlow::Undefined,
            KiControlFlow::Throw(e) => KiControlFlow::Throw(e),
        }
    }
}
