use crate::*;
use monad::Monad;

#[must_use]
pub enum ProjStageChangeM<T> {
    Ok(T),
}

impl<T> Monad for ProjStageChangeM<T> {}

pub struct ProjProjectChangeR<T> {
    phantom: PhantomData<T>,
}

impl<T> std::ops::Try for ProjStageChangeM<T> {
    type Output = T;

    type Residual = ProjProjectChangeR<T>;

    fn from_output(cont: Self::Output) -> Self {
        ProjStageChangeM::Ok(cont)
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            ProjStageChangeM::Ok(cont) => std::ops::ControlFlow::Continue(cont),
        }
    }
}

impl<T> std::ops::FromResidual<ProjProjectChangeR<T>> for ProjStageChangeM<T> {
    fn from_residual(residual: ProjProjectChangeR<T>) -> Self {
        todo!()
    }
}
