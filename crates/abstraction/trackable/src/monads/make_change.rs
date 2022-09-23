use crate::*;
use ::monad::Monad;

#[must_use]
pub enum TrackableMakeChangeM<P, T> {
    Ok {
        cont: T,
        phantom_state: PhantomData<P>,
    },
}

impl<P, T> Monad for TrackableMakeChangeM<P, T> {}

pub struct TrackableMakeChangeR<P> {
    phantom_state: PhantomData<P>,
}

// implementation details

impl<P, T> Default for TrackableMakeChangeM<P, T>
where
    T: Default,
{
    fn default() -> Self {
        TrackableMakeChangeM::Ok {
            cont: Default::default(),
            phantom_state: PhantomData,
        }
    }
}

impl<P, T> std::ops::Try for TrackableMakeChangeM<P, T> {
    type Output = T;

    type Residual = TrackableMakeChangeR<P>;

    fn from_output(cont: Self::Output) -> Self {
        TrackableMakeChangeM::Ok {
            cont,
            phantom_state: PhantomData,
        }
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            TrackableMakeChangeM::Ok { cont, .. } => std::ops::ControlFlow::Continue(cont),
        }
    }
}

impl<P, Q, T> std::ops::FromResidual<TrackableMakeChangeR<P>> for TrackableMakeChangeM<Q, T> {
    fn from_residual(residual: TrackableMakeChangeR<P>) -> Self {
        todo!()
    }
}
