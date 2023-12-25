use std::{
    convert::Infallible,
    ops::{FromResidual, Try},
};

use crate::value::IsValue;

#[derive(Debug, PartialEq, Eq)]
pub enum ValControlFlow<C, B, E> {
    Continue(C),
    LoopContinue,
    LoopBreak(B),
    Return(B),
    Undefined,
    Err(E),
}

impl<C, B, E> std::ops::Residual<C> for ValControlFlow<Infallible, B, E> {
    type TryType = ValControlFlow<C, B, E>;
}

impl<C, B, E> std::ops::Try for ValControlFlow<C, B, E> {
    type Output = C;

    type Residual = ValControlFlow<Infallible, B, E>;

    fn from_output(output: Self::Output) -> Self {
        todo!()
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            ValControlFlow::Continue(c) => std::ops::ControlFlow::Continue(c),
            ValControlFlow::LoopContinue => {
                std::ops::ControlFlow::Break(ValControlFlow::LoopContinue)
            }
            ValControlFlow::LoopBreak(b) => {
                std::ops::ControlFlow::Break(ValControlFlow::LoopBreak(b))
            }
            ValControlFlow::Return(b) => std::ops::ControlFlow::Break(ValControlFlow::LoopBreak(b)),
            ValControlFlow::Undefined => std::ops::ControlFlow::Break(ValControlFlow::Undefined),
            ValControlFlow::Err(e) => std::ops::ControlFlow::Break(ValControlFlow::Err(e)),
        }
    }
}

impl<C, B, E> FromResidual<ValControlFlow<Infallible, B, E>> for ValControlFlow<C, B, E> {
    fn from_residual(residual: ValControlFlow<Infallible, B, E>) -> Self {
        match residual {
            ValControlFlow::Continue(_) => unreachable!(),
            ValControlFlow::LoopContinue => ValControlFlow::LoopContinue,
            ValControlFlow::LoopBreak(b) => ValControlFlow::LoopBreak(b),
            ValControlFlow::Return(b) => ValControlFlow::Return(b),
            ValControlFlow::Undefined => ValControlFlow::Undefined,
            ValControlFlow::Err(e) => ValControlFlow::Err(e),
        }
    }
}

impl<C, B, E> FromResidual<Result<Infallible, E>> for ValControlFlow<C, B, E> {
    fn from_residual(residual: Result<Infallible, E>) -> Self {
        match residual {
            Ok(_) => unreachable!(),
            Err(e) => ValControlFlow::Err(e),
        }
    }
}

impl<C1, C2: FromIterator<C1>, B, E> std::iter::FromIterator<ValControlFlow<C1, B, E>>
    for ValControlFlow<C2, B, E>
{
    fn from_iter<T: IntoIterator<Item = ValControlFlow<C1, B, E>>>(iter: T) -> Self {
        match iter
            .into_iter()
            .map(|item| match item.branch() {
                std::ops::ControlFlow::Continue(c1) => Ok(c1),
                std::ops::ControlFlow::Break(residual) => Err(residual),
            })
            .collect::<Result<C2, ValControlFlow<Infallible, B, E>>>()
        {
            Ok(c2) => ValControlFlow::Continue(c2),
            Err(residual) => ValControlFlow::from_residual(residual),
        }
    }
}

impl<Value, E> ValControlFlow<Value, Value, E>
where
    Value: IsValue,
    E: 'static,
{
    pub unsafe fn share_unchecked(&self) -> Self {
        let slf: &'static Self = std::mem::transmute(self);
        slf.share()
    }

    fn share(&'static self) -> Self {
        match self {
            ValControlFlow::Continue(value) => ValControlFlow::Continue(value.share()),
            ValControlFlow::LoopContinue => todo!(),
            ValControlFlow::LoopBreak(_) => todo!(),
            ValControlFlow::Return(_) => todo!(),
            ValControlFlow::Undefined => todo!(),
            ValControlFlow::Err(_) => todo!(),
        }
    }

    pub(crate) fn unwrap(self) -> Value {
        match self.branch() {
            std::ops::ControlFlow::Continue(value) => value,
            std::ops::ControlFlow::Break(_) => panic!(),
        }
    }
}
