use crate::*;
use husky_vm::__VMError;
use monad::Monad;
use std::ops::FromResidual;
use trackable::{
    Trackable, TrackableApplyChangeR, TrackableAtom, TrackableMakeChangeR, TrackableTakeChangeR,
    TrackableVec,
};

#[must_use]
pub enum HuskyDebugtimeTakeChangeM<T> {
    Ok(T),
    OtherworldlyErr(__VMError),
}
impl<T> Monad for HuskyDebugtimeTakeChangeM<T> {}

impl HuskyDebugtime {
    pub(crate) fn take_change(&mut self) -> HuskyDebugtimeTakeChangeM<DebugtimeStateChange> {
        HuskyDebugtimeTakeChangeM::Ok(self.state.take_change()?)
    }
}

// implementation details

pub enum HuskyDebugtimeTakeChangeR {
    OtherworldlyErr(__VMError),
}

impl<T> FromResidual<TrackableApplyChangeR<TrackableVec<TraceNode>>>
    for HuskyDebugtimeTakeChangeM<T>
{
    fn from_residual(residual: TrackableApplyChangeR<TrackableVec<TraceNode>>) -> Self {
        todo!()
    }
}

impl FromResidual<TrackableTakeChangeR<HuskyDebugtimeState>>
    for HuskyDebugtimeTakeChangeM<DebugtimeStateChange>
{
    fn from_residual(residual: TrackableTakeChangeR<HuskyDebugtimeState>) -> Self {
        todo!()
    }
}

impl<T> FromResidual<HuskyDebugtimeTakeChangeR> for HuskyDebugtimeTakeChangeM<T> {
    fn from_residual(residual: HuskyDebugtimeTakeChangeR) -> Self {
        todo!()
    }
}

impl<T> FromResidual<HuskyDebugtimeUpdateR> for HuskyDebugtimeTakeChangeM<T> {
    fn from_residual(residual: HuskyDebugtimeUpdateR) -> Self {
        todo!()
    }
}

impl<T> FromResidual<TrackableMakeChangeR<TrackableAtom<Restriction>>>
    for HuskyDebugtimeTakeChangeM<T>
{
    fn from_residual(residual: TrackableMakeChangeR<TrackableAtom<Restriction>>) -> Self {
        todo!()
    }
}

impl<T> std::ops::Try for HuskyDebugtimeTakeChangeM<T> {
    type Output = T;

    type Residual = HuskyDebugtimeTakeChangeR;

    fn from_output(output: Self::Output) -> Self {
        HuskyDebugtimeTakeChangeM::Ok(output)
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            HuskyDebugtimeTakeChangeM::Ok(cont) => std::ops::ControlFlow::Continue(cont),
            HuskyDebugtimeTakeChangeM::OtherworldlyErr(e) => {
                std::ops::ControlFlow::Break(HuskyDebugtimeTakeChangeR::OtherworldlyErr(e))
            }
        }
    }
}
