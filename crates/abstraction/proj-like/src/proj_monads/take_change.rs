use crate::*;
use monad::Monad;

#[must_use]
pub enum ProjTakeChangeM<T: Proj> {
    Ok(T::Change),
}

impl<T> Monad for ProjTakeChangeM<T> where T: Proj {}

pub struct ProjTakeChangeR<T> {
    phantom: PhantomData<T>,
}

impl<T> std::ops::Try for ProjTakeChangeM<T>
where
    T: Proj,
{
    type Output = T::Change;

    type Residual = ProjTakeChangeR<T>;

    fn from_output(cont: Self::Output) -> Self {
        ProjTakeChangeM::Ok(cont)
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            ProjTakeChangeM::Ok(cont) => std::ops::ControlFlow::Continue(cont),
        }
    }
}

impl<T> std::ops::FromResidual<ProjTakeChangeR<T>> for ProjTakeChangeM<T>
where
    T: Proj,
{
    fn from_residual(residual: ProjTakeChangeR<T>) -> Self {
        todo!()
    }
}
