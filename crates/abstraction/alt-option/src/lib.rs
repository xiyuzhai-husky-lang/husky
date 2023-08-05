#![feature(try_trait_v2)]
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

    fn from_output(output: Self::Output) -> Self {
        AltOption::AltNone
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            AltOption::AltSome(t) => std::ops::ControlFlow::Break(AltOptionR(t)),
            AltOption::AltNone => std::ops::ControlFlow::Continue(()),
        }
    }
}

impl<T> std::ops::FromResidual<AltOptionR<T>> for AltOption<T> {
    fn from_residual(residual: AltOptionR<T>) -> Self {
        AltOption::AltSome(residual.0)
    }
}
