use crate::*;
use ::monad::Monad;

#[must_use]
pub enum TrackableUpdateM<P, T> {
    Ok {
        cont: T,
        phantom_state: PhantomData<P>,
    },
}

impl<P, T> Monad for TrackableUpdateM<P, T> {}

pub struct TrackableMakeChangeR<P> {
    phantom_state: PhantomData<P>,
}

impl<P, T> std::ops::Try for TrackableUpdateM<P, T> {
    type Output = T;

    type Residual = TrackableMakeChangeR<P>;

    fn from_output(cont: Self::Output) -> Self {
        TrackableUpdateM::Ok {
            cont,
            phantom_state: PhantomData,
        }
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            TrackableUpdateM::Ok { cont, .. } => std::ops::ControlFlow::Continue(cont),
        }
    }
}

impl<P, T> std::ops::FromResidual<TrackableMakeChangeR<P>> for TrackableUpdateM<P, T> {
    fn from_residual(residual: TrackableMakeChangeR<P>) -> Self {
        todo!()
    }
}
