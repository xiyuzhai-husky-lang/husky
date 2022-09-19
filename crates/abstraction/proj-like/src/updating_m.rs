use crate::*;
use monad::Monad;

#[must_use]
pub enum ProjUpdatingM<State, T> {
    Ok {
        cont: T,
        phantom_state: PhantomData<State>,
    },
}

impl<P, T> Monad for ProjUpdatingM<P, T> {}

pub struct ProjUpdatingR<State, Continue> {
    phantom_state: PhantomData<State>,
    phantom_cont: PhantomData<Continue>,
}

impl<State, T> std::ops::Try for ProjUpdatingM<State, T> {
    type Output = T;

    type Residual = ProjUpdatingR<State, T>;

    fn from_output(cont: Self::Output) -> Self {
        ProjUpdatingM::Ok {
            cont,
            phantom_state: PhantomData,
        }
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            ProjUpdatingM::Ok { cont, .. } => std::ops::ControlFlow::Continue(cont),
        }
    }
}

impl<P, T> std::ops::FromResidual<ProjUpdatingR<P, T>> for ProjUpdatingM<P, T> {
    fn from_residual(residual: ProjUpdatingR<P, T>) -> Self {
        todo!()
    }
}
