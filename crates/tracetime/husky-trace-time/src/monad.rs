use std::ops::{FromResidual, Try as Yield};

use husky_vm::__VMError;

use crate::HuskyTracetime;

pub(crate) struct TracetimeMonad<'a, T> {
    tracetime: &'a mut HuskyTracetime,
    value: T,
    variant: TracetimeMonadVariant,
}
pub(crate) enum TracetimeMonadVariant {
    Success,
    Failure,
    Uncertain,
}

impl<'a, T> Yield for TracetimeMonad<'a, T> {
    type Output = T;

    type Residual = __VMError;

    fn from_output(output: Self::Output) -> Self {
        todo!()
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        todo!()
    }
}

impl<'a, T> FromResidual<__VMError> for TracetimeMonad<'a, T> {
    fn from_residual(residual: __VMError) -> Self {
        todo!()
    }
}

impl HuskyTracetime {
    fn pure<T>(&mut self, t: T) -> TracetimeMonad<T> {
        todo!()
    }
}
