#![feature(try_trait_v2)]
pub enum AltOption<T> {
    Some(T),
    None,
}

pub struct AltOptionR<T>(T);

impl<T> std::ops::Try for AltOption<T> {
    type Output = ();

    type Residual = AltOptionR<T>;

    fn from_output(output: Self::Output) -> Self {
        AltOption::None
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            AltOption::Some(t) => std::ops::ControlFlow::Break(AltOptionR(t)),
            AltOption::None => std::ops::ControlFlow::Continue(()),
        }
    }
}

impl<T> std::ops::FromResidual<AltOptionR<T>> for AltOption<T> {
    fn from_residual(residual: AltOptionR<T>) -> Self {
        AltOption::Some(residual.0)
    }
}
