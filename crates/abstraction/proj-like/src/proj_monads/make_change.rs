use crate::*;
use ::monad::Monad;

#[must_use]
pub enum ProjMakeChangeM<State, T> {
    Ok {
        cont: T,
        phantom_state: PhantomData<State>,
    },
}

impl<P, T> Monad for ProjMakeChangeM<P, T> {}

pub struct ProjMakeChangeR<State, Continue> {
    phantom_state: PhantomData<State>,
    phantom_cont: PhantomData<Continue>,
}

impl<State, T> std::ops::Try for ProjMakeChangeM<State, T> {
    type Output = T;

    type Residual = ProjMakeChangeR<State, T>;

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

impl<P, T> std::ops::FromResidual<ProjMakeChangeR<P, T>> for ProjMakeChangeM<P, T> {
    fn from_residual(residual: ProjMakeChangeR<P, T>) -> Self {
        todo!()
    }
}
