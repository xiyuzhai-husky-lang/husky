use std::ops::{FromResidual, Try};

struct WriteFile;
struct WriteFileResidual;

impl FromResidual<WriteFileResidual> for WriteFile {
    fn from_residual(residual: WriteFileResidual) -> Self {
        Self
    }
}

impl Try for WriteFile {
    type Output = ();

    type Residual = WriteFileResidual;

    fn from_output(output: Self::Output) -> Self {
        Self
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        std::ops::ControlFlow::Continue(())
    }
}

trait Monad: std::ops::Try {}

trait FromMonad<T: Monad>: FromResidual<T::Residual> {}
