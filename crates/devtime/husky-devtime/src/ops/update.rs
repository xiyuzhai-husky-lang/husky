use crate::*;

use monad::Monad;
use std::ops::FromResidual;
use trackable::{TrackableAtom, TrackableMakeChangeR, TrackableMap, TrackableVec};

#[must_use]
pub(crate) enum DevtimeUpdateM<T> {
    Ok(T),
    OtherworldlyErr(__VMError),
}

pub enum DevtimeUpdateR {
    OtherworldlyErr(__VMError),
}

impl<T> Monad for DevtimeUpdateM<T> {}

impl<T> DevtimeUpdateM<T> {
    pub(crate) fn result(self) -> DevtimeUpdateM<__VMResult<T>> {
        match self {
            DevtimeUpdateM::Ok(output) => DevtimeUpdateM::Ok(Ok(output)),
            DevtimeUpdateM::OtherworldlyErr(_) => todo!(),
        }
    }
}
impl<T> DevtimeTakeChangeM<T> {
    pub fn result(self) -> DevtimeTakeChangeM<__VMResult<T>> {
        match self {
            DevtimeTakeChangeM::Ok(t) => DevtimeTakeChangeM::Ok(Ok(t)),
            DevtimeTakeChangeM::OtherworldlyErr(e) => DevtimeTakeChangeM::Ok(Err(e)),
        }
    }
}

impl Devtime {
    pub(crate) fn update(&mut self) -> DevtimeUpdateM<()> {
        match self.try_update().result()? {
            Ok(()) => DevtimeUpdateM::Ok(()),
            Err(e) => match e.variant() {
                __VMErrorVariant::Normal => todo!(),
                __VMErrorVariant::FromBatch { sample_id } => {
                    self.state.update_presentation(|presentation| {
                        presentation.set_specific(SampleId(*sample_id))
                    })?;
                    self.update()
                }
            },
        }
    }

    fn try_update(&mut self) -> DevtimeUpdateM<()> {
        self.update_figure_canvases()?;
        self.update_figure_controls()?;
        self.update_trace_stalks()?;
        self.update_trace_statss()
    }
}

impl<T> FromResidual<Result<std::convert::Infallible, __VMError>> for DevtimeUpdateM<T> {
    fn from_residual(_residual: Result<std::convert::Infallible, __VMError>) -> Self {
        todo!()
    }
}

// todo: refine this
impl<T> FromResidual<ServerTraceStateUpdateR<T>> for DevtimeUpdateM<T> {
    fn from_residual(_residual: ServerTraceStateUpdateR<T>) -> Self {
        todo!()
    }
}

impl<T> FromResidual<DevtimeUpdateR> for DevtimeUpdateM<T> {
    fn from_residual(_residual: DevtimeUpdateR) -> Self {
        todo!()
    }
}

impl<T> FromResidual<TrackableMakeChangeR<TrackableMap<TraceStatsKey, Option<TraceStats>>>>
    for DevtimeUpdateM<T>
{
    fn from_residual(
        _residual: TrackableMakeChangeR<TrackableMap<TraceStatsKey, Option<TraceStats>>>,
    ) -> Self {
        todo!()
    }
}

impl<T> FromResidual<TrackableMakeChangeR<TrackableVec<TraceNode>>> for DevtimeUpdateM<T> {
    fn from_residual(_residual: TrackableMakeChangeR<TrackableVec<TraceNode>>) -> Self {
        todo!()
    }
}

impl<T> FromResidual<TrackableMakeChangeR<TrackableAtom<Presentation>>> for DevtimeUpdateM<T> {
    fn from_residual(_residual: TrackableMakeChangeR<TrackableAtom<Presentation>>) -> Self {
        todo!()
    }
}

impl<T> FromResidual<TrackableMakeChangeR<DevtimeState>> for DevtimeUpdateM<T> {
    fn from_residual(_residual: TrackableMakeChangeR<DevtimeState>) -> Self {
        todo!()
    }
}
impl<T>
    FromResidual<
        TrackableMakeChangeR<TrackableMap<GenericFigureCanvasKey, GenericFigureCanvasData>>,
    > for DevtimeUpdateM<T>
{
    fn from_residual(
        _residual: TrackableMakeChangeR<
            TrackableMap<GenericFigureCanvasKey, GenericFigureCanvasData>,
        >,
    ) -> Self {
        todo!()
    }
}

impl<T>
    FromResidual<
        TrackableMakeChangeR<TrackableMap<SpecificFigureCanvasKey, SpecificFigureCanvasData>>,
    > for DevtimeUpdateM<T>
{
    fn from_residual(
        _residual: TrackableMakeChangeR<
            TrackableMap<SpecificFigureCanvasKey, SpecificFigureCanvasData>,
        >,
    ) -> Self {
        todo!()
    }
}

impl<T> FromResidual<TrackableMakeChangeR<TrackableMap<FigureControlKey, FigureControlData>>>
    for DevtimeUpdateM<T>
{
    fn from_residual(
        _residual: TrackableMakeChangeR<TrackableMap<FigureControlKey, FigureControlData>>,
    ) -> Self {
        todo!()
    }
}

impl<T> std::ops::Try for DevtimeUpdateM<T> {
    type Output = T;

    type Residual = DevtimeUpdateR;

    fn from_output(output: Self::Output) -> Self {
        DevtimeUpdateM::Ok(output)
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            DevtimeUpdateM::Ok(cont) => std::ops::ControlFlow::Continue(cont),
            DevtimeUpdateM::OtherworldlyErr(e) => {
                std::ops::ControlFlow::Break(DevtimeUpdateR::OtherworldlyErr(e))
            }
        }
    }
}
