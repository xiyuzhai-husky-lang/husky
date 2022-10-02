use crate::*;
use husky_vm::__VMError;
use monad::Monad;
use std::ops::FromResidual;
use trackable::{
    Trackable, TrackableApplyChangeR, TrackableAtom, TrackableMakeChangeR, TrackableTakeChangeR,
    TrackableVec,
};

#[must_use]
pub enum HuskyDevtimeTakeChangeM<T> {
    Ok(T),
    OtherworldlyErr(__VMError),
}
impl<T> Monad for HuskyDevtimeTakeChangeM<T> {}

impl HuskyDevtime {
    pub(crate) fn take_change(&mut self) -> HuskyDevtimeTakeChangeM<DevtimeStateChange> {
        HuskyDevtimeTakeChangeM::Ok(self.state.take_change()?)
    }
}

// implementation details

pub enum HuskyDevtimeTakeChangeR {
    OtherworldlyErr(__VMError),
}

impl<T> FromResidual<TrackableApplyChangeR<TrackableVec<TraceNode>>>
    for HuskyDevtimeTakeChangeM<T>
{
    fn from_residual(residual: TrackableApplyChangeR<TrackableVec<TraceNode>>) -> Self {
        todo!()
    }
}

impl FromResidual<TrackableTakeChangeR<HuskyDevtimeState>>
    for HuskyDevtimeTakeChangeM<DevtimeStateChange>
{
    fn from_residual(residual: TrackableTakeChangeR<HuskyDevtimeState>) -> Self {
        todo!()
    }
}

impl<T> FromResidual<HuskyDevtimeTakeChangeR> for HuskyDevtimeTakeChangeM<T> {
    fn from_residual(residual: HuskyDevtimeTakeChangeR) -> Self {
        todo!()
    }
}

impl<T> FromResidual<HuskyDevtimeUpdateR> for HuskyDevtimeTakeChangeM<T> {
    fn from_residual(residual: HuskyDevtimeUpdateR) -> Self {
        todo!()
    }
}

impl<T> FromResidual<TrackableMakeChangeR<TrackableAtom<Restriction>>>
    for HuskyDevtimeTakeChangeM<T>
{
    fn from_residual(residual: TrackableMakeChangeR<TrackableAtom<Restriction>>) -> Self {
        todo!()
    }
}

impl<T> std::ops::Try for HuskyDevtimeTakeChangeM<T> {
    type Output = T;

    type Residual = HuskyDevtimeTakeChangeR;

    fn from_output(output: Self::Output) -> Self {
        HuskyDevtimeTakeChangeM::Ok(output)
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            HuskyDevtimeTakeChangeM::Ok(cont) => std::ops::ControlFlow::Continue(cont),
            HuskyDevtimeTakeChangeM::OtherworldlyErr(e) => {
                std::ops::ControlFlow::Break(HuskyDevtimeTakeChangeR::OtherworldlyErr(e))
            }
        }
    }
}
