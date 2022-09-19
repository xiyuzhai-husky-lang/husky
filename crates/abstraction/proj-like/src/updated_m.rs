use crate::*;
use monad::Monad;

#[must_use]
pub enum ProjUpdatedM<T> {
    Ok(T),
}

impl<T> Monad for ProjUpdatedM<T> {}

pub struct ProjUpdatedR<T> {
    phantom: PhantomData<T>,
}

impl<T> std::ops::Try for ProjUpdatedM<T> {
    type Output = T;

    type Residual = ProjUpdatedR<T>;

    fn from_output(cont: Self::Output) -> Self {
        ProjUpdatedM::Ok(cont)
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            ProjUpdatedM::Ok(cont) => std::ops::ControlFlow::Continue(cont),
        }
    }
}

impl<T> std::ops::FromResidual<ProjUpdatedR<T>> for ProjUpdatedM<T> {
    fn from_residual(residual: ProjUpdatedR<T>) -> Self {
        todo!()
    }
}
