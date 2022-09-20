use crate::*;
use ::monad::Monad;

#[must_use]
pub enum ProjMakeChangeM<P, T> {
    Ok {
        cont: T,
        phantom_state: PhantomData<P>,
    },
}

impl<P, T> Monad for ProjMakeChangeM<P, T> {}

pub struct ProjMakeChangeR<P> {
    phantom_state: PhantomData<P>,
}

impl<P, T> std::ops::Try for ProjMakeChangeM<P, T> {
    type Output = T;

    type Residual = ProjMakeChangeR<P>;

    fn from_output(cont: Self::Output) -> Self {
        ProjMakeChangeM::Ok {
            cont,
            phantom_state: PhantomData,
        }
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            ProjMakeChangeM::Ok { cont, .. } => std::ops::ControlFlow::Continue(cont),
        }
    }
}

impl<P, T> std::ops::FromResidual<ProjMakeChangeR<P>> for ProjMakeChangeM<P, T> {
    fn from_residual(residual: ProjMakeChangeR<P>) -> Self {
        todo!()
    }
}
