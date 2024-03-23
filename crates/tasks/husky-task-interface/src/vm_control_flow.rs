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

pub type ValuePresentationVmControlFlow =
    VmControlFlow<ValuePresentation, ValuePresentation, ValuePresentation>;
pub type LinkageImplVmControlFlow<LinkageImpl> = VmControlFlow<
    <LinkageImpl as IsLinkageImpl>::Value,
    <LinkageImpl as IsLinkageImpl>::Value,
    <LinkageImpl as IsLinkageImpl>::Exception,
>;

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
