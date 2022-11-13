use crate::*;
use monad::Monad;

#[must_use]
pub enum TrackableTakeChangeM<T: Trackable> {
    Ok(T::Change),
}

impl<T> Monad for TrackableTakeChangeM<T> where T: Trackable {}

pub struct TrackableTakeChangeR<T> {
    phantom: PhantomData<T>,
}

impl<T> std::ops::Try for TrackableTakeChangeM<T>
where
    T: Trackable,
{
    type Output = T::Change;

    type Residual = TrackableTakeChangeR<T>;

    fn from_output(cont: Self::Output) -> Self {
        TrackableTakeChangeM::Ok(cont)
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            TrackableTakeChangeM::Ok(cont) => std::ops::ControlFlow::Continue(cont),
        }
    }
}

impl<T, S> std::ops::FromResidual<TrackableTakeChangeR<T>> for TrackableTakeChangeM<S>
where
    T: Trackable,
    S: Trackable,
{
    fn from_residual(_residual: TrackableTakeChangeR<T>) -> Self {
        todo!()
    }
}
