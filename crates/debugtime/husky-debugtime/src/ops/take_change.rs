use crate::*;
use husky_vm::__VMError;
use monad::Monad;
use std::ops::FromResidual;
use trackable::{
    Trackable, TrackableApplyChangeR, TrackableAtom, TrackableMakeChangeR, TrackableTakeChangeR,
    TrackableVec,
};

#[must_use]
pub enum DebugtimeTakeChangeM<T> {
    Ok(T),
    OtherworldlyErr(__VMError),
}
impl<T> Monad for DebugtimeTakeChangeM<T> {}

impl Debugtime {
    pub(crate) fn take_change(&mut self) -> DebugtimeTakeChangeM<DebugtimeStateChange> {
        DebugtimeTakeChangeM::Ok(self.state.take_change()?)
    }
}

// implementation details

pub enum DebugtimeTakeChangeR {
    OtherworldlyErr(__VMError),
}

impl<T> FromResidual<TrackableApplyChangeR<TrackableVec<TraceNode>>> for DebugtimeTakeChangeM<T> {
    fn from_residual(_residual: TrackableApplyChangeR<TrackableVec<TraceNode>>) -> Self {
        todo!()
    }
}

impl FromResidual<TrackableTakeChangeR<DebugtimeState>>
    for DebugtimeTakeChangeM<DebugtimeStateChange>
{
    fn from_residual(_residual: TrackableTakeChangeR<DebugtimeState>) -> Self {
        todo!()
    }
}

impl<T> FromResidual<DebugtimeTakeChangeR> for DebugtimeTakeChangeM<T> {
    fn from_residual(_residual: DebugtimeTakeChangeR) -> Self {
        todo!()
    }
}

impl<T> FromResidual<DebugtimeUpdateR> for DebugtimeTakeChangeM<T> {
    fn from_residual(_residual: DebugtimeUpdateR) -> Self {
        todo!()
    }
}

impl<T> FromResidual<TrackableMakeChangeR<DebugtimeState>> for DebugtimeTakeChangeM<T> {
    fn from_residual(_residual: TrackableMakeChangeR<DebugtimeState>) -> Self {
        todo!()
    }
}

impl<T> FromResidual<TrackableMakeChangeR<TrackableAtom<Presentation>>>
    for DebugtimeTakeChangeM<T>
{
    fn from_residual(_residual: TrackableMakeChangeR<TrackableAtom<Presentation>>) -> Self {
        todo!()
    }
}

impl<T> std::ops::Try for DebugtimeTakeChangeM<T> {
    type Output = T;

    type Residual = DebugtimeTakeChangeR;

    fn from_output(output: Self::Output) -> Self {
        DebugtimeTakeChangeM::Ok(output)
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            DebugtimeTakeChangeM::Ok(cont) => std::ops::ControlFlow::Continue(cont),
            DebugtimeTakeChangeM::OtherworldlyErr(e) => {
                std::ops::ControlFlow::Break(DebugtimeTakeChangeR::OtherworldlyErr(e))
            }
        }
    }
}
