use std::{
    convert::Infallible,
    ops::{FromResidual, Try},
};

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

impl<C, B, E> std::ops::FromResidual<ValControlFlow<Infallible, B, E>> for ValControlFlow<C, B, E> {
    fn from_residual(residual: ValControlFlow<Infallible, B, E>) -> Self {
        todo!()
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