use crate::*;
use ::monad::Monad;

#[must_use]
pub enum ProjUpdateM<P, T> {
    Ok {
        cont: T,
        phantom_state: PhantomData<P>,
    },
}

impl<P, T> Monad for ProjUpdateM<P, T> {}

pub struct ProjUpdateR<P> {
    phantom_state: PhantomData<P>,
}

impl<P, T> std::ops::Try for ProjUpdateM<P, T> {
    type Output = T;

    type Residual = ProjUpdateR<P>;

    fn from_output(cont: Self::Output) -> Self {
        ProjUpdateM::Ok {
            cont,
            phantom_state: PhantomData,
        }
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            ProjUpdateM::Ok { cont, .. } => std::ops::ControlFlow::Continue(cont),
        }
    }
}

impl<P, T> std::ops::FromResidual<ProjUpdateR<P>> for ProjUpdateM<P, T> {
    fn from_residual(residual: ProjUpdateR<P>) -> Self {
        todo!()
    }
}
