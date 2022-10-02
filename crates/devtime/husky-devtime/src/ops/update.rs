use crate::*;
use husky_entity_kind::EntityKind;
use monad::Monad;
use std::{ops::FromResidual, time::Instant};
use trackable::{TrackableAtom, TrackableMakeChangeR, TrackableMap};

#[must_use]
pub(crate) enum HuskyDevtimeUpdateM<T> {
    Ok(T),
    OtherworldlyErr(__VMError),
}

pub enum HuskyDevtimeUpdateR {
    OtherworldlyErr(__VMError),
}

impl<T> Monad for HuskyDevtimeUpdateM<T> {}

impl<T> HuskyDevtimeUpdateM<T> {
    pub(crate) fn result(self) -> HuskyDevtimeUpdateM<__VMResult<T>> {
        match self {
            HuskyDevtimeUpdateM::Ok(output) => HuskyDevtimeUpdateM::Ok(Ok(output)),
            HuskyDevtimeUpdateM::OtherworldlyErr(_) => todo!(),
        }
    }
}
impl<T> HuskyDevtimeTakeChangeM<T> {
    pub fn result(self) -> HuskyDevtimeTakeChangeM<__VMResult<T>> {
        match self {
            HuskyDevtimeTakeChangeM::Ok(t) => HuskyDevtimeTakeChangeM::Ok(Ok(t)),
            HuskyDevtimeTakeChangeM::OtherworldlyErr(e) => HuskyDevtimeTakeChangeM::Ok(Err(e)),
        }
    }
}

impl HuskyDevtime {
    pub(crate) fn update(&mut self) -> HuskyDevtimeUpdateM<()> {
        match self.try_update().result()? {
            Ok(()) => HuskyDevtimeUpdateM::Ok(()),
            Err(e) => match e.variant() {
                __VMErrorVariant::Normal => todo!(),
                __VMErrorVariant::FromBatch { sample_id } => {
                    self.state
                        .restriction
                        .update(|restriction| restriction.set_specific(SampleId(*sample_id)));
                    self.update()
                }
            },
        }
    }

    fn try_update(&mut self) -> HuskyDevtimeUpdateM<()> {
        self.update_figure_canvases()?;
        self.update_figure_controls()?;
        self.update_trace_stalks()?;
        self.update_trace_statss()
    }
}

impl<T> FromResidual<Result<std::convert::Infallible, __VMError>> for HuskyDevtimeUpdateM<T> {
    fn from_residual(residual: Result<std::convert::Infallible, __VMError>) -> Self {
        todo!()
    }
}

impl<T> FromResidual<HuskyDevtimeUpdateR> for HuskyDevtimeUpdateM<T> {
    fn from_residual(residual: HuskyDevtimeUpdateR) -> Self {
        todo!()
    }
}

impl<T> FromResidual<TrackableMakeChangeR<TrackableAtom<Restriction>>> for HuskyDevtimeUpdateM<T> {
    fn from_residual(residual: TrackableMakeChangeR<TrackableAtom<Restriction>>) -> Self {
        todo!()
    }
}

impl<T> FromResidual<TrackableMakeChangeR<TrackableMap<FigureCanvasKey, FigureCanvasData>>>
    for HuskyDevtimeUpdateM<T>
{
    fn from_residual(
        residual: TrackableMakeChangeR<TrackableMap<FigureCanvasKey, FigureCanvasData>>,
    ) -> Self {
        todo!()
    }
}

impl<T> FromResidual<TrackableMakeChangeR<TrackableMap<FigureControlKey, FigureControlData>>>
    for HuskyDevtimeUpdateM<T>
{
    fn from_residual(
        residual: TrackableMakeChangeR<TrackableMap<FigureControlKey, FigureControlData>>,
    ) -> Self {
        todo!()
    }
}

impl<T> std::ops::Try for HuskyDevtimeUpdateM<T> {
    type Output = T;

    type Residual = HuskyDevtimeUpdateR;

    fn from_output(output: Self::Output) -> Self {
        HuskyDevtimeUpdateM::Ok(output)
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            HuskyDevtimeUpdateM::Ok(cont) => std::ops::ControlFlow::Continue(cont),
            HuskyDevtimeUpdateM::OtherworldlyErr(e) => {
                std::ops::ControlFlow::Break(HuskyDevtimeUpdateR::OtherworldlyErr(e))
            }
        }
    }
}
