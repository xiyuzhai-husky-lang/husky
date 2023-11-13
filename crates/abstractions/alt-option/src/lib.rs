#![feature(try_trait_v2)]
use std::convert::Infallible;

pub use AltOption::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum AltOption<T> {
    AltSome(T),
    AltNone,
}

pub struct AltOptionR<T>(T);

impl<T> std::ops::Try for AltOption<T> {
    type Output = ();

    type Residual = AltOptionR<T>;

    fn from_output(_output: Self::Output) -> Self {
        AltNone
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            AltSome(t) => std::ops::ControlFlow::Break(AltOptionR(t)),
            AltNone => std::ops::ControlFlow::Continue(()),
        }
    }
}

impl<T> std::ops::FromResidual<AltOptionR<T>> for AltOption<T> {
    fn from_residual(residual: AltOptionR<T>) -> Self {
        AltOption::AltSome(residual.0)
    }
}

impl<T> std::ops::FromResidual<Option<Infallible>> for AltOption<T> {
    fn from_residual(_residual: Option<Infallible>) -> Self {
        AltNone
    }
}
