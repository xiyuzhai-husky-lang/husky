use crate::*;
use husky_vm::__VMError;
use std::ops::FromResidual;
use trackable::{
    Trackable, TrackableApplyChangeR, TrackableAtom, TrackableMakeChangeR, TrackableTakeChangeR,
    TrackableVec,
};

#[must_use]
pub enum DevtimeTakeChangeM<T> {
    Ok(T),
    OtherworldlyErr(__VMError),
}

impl Devtime {
    pub(crate) fn take_change(&mut self) -> DevtimeStateChange {
        self.state.take_change()
    }
}

// implementation details

pub enum DevtimeTakeChangeR {
    OtherworldlyErr(__VMError),
}

impl<T> FromResidual<TrackableApplyChangeR<TrackableVec<TraceNode>>> for DevtimeTakeChangeM<T> {
    fn from_residual(_residual: TrackableApplyChangeR<TrackableVec<TraceNode>>) -> Self {
        todo!()
    }
}

impl FromResidual<TrackableTakeChangeR<DevtimeState>> for DevtimeTakeChangeM<DevtimeStateChange> {
    fn from_residual(_residual: TrackableTakeChangeR<DevtimeState>) -> Self {
        todo!()
    }
}

impl<T> FromResidual<DevtimeTakeChangeR> for DevtimeTakeChangeM<T> {
    fn from_residual(_residual: DevtimeTakeChangeR) -> Self {
        todo!()
    }
}

impl<T> FromResidual<DevtimeUpdateR> for DevtimeTakeChangeM<T> {
    fn from_residual(_residual: DevtimeUpdateR) -> Self {
        todo!()
    }
}

impl<T> FromResidual<TrackableMakeChangeR<DevtimeState>> for DevtimeTakeChangeM<T> {
    fn from_residual(_residual: TrackableMakeChangeR<DevtimeState>) -> Self {
        todo!()
    }
}

impl<T> FromResidual<TrackableMakeChangeR<TrackableAtom<Presentation>>> for DevtimeTakeChangeM<T> {
    fn from_residual(_residual: TrackableMakeChangeR<TrackableAtom<Presentation>>) -> Self {
        todo!()
    }
}

impl<T> std::ops::Try for DevtimeTakeChangeM<T> {
    type Output = T;

    type Residual = DevtimeTakeChangeR;

    fn from_output(output: Self::Output) -> Self {
        DevtimeTakeChangeM::Ok(output)
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            DevtimeTakeChangeM::Ok(cont) => std::ops::ControlFlow::Continue(cont),
            DevtimeTakeChangeM::OtherworldlyErr(e) => {
                std::ops::ControlFlow::Break(DevtimeTakeChangeR::OtherworldlyErr(e))
            }
        }
    }
}
