use crate::*;

use monad::Monad;
use std::ops::FromResidual;
use trackable::{TrackableAtom, TrackableMakeChangeR, TrackableMap, TrackableVec};

#[must_use]
pub(crate) enum DebugtimeUpdateM<T> {
    Ok(T),
    OtherworldlyErr(__VMError),
}

pub enum DebugtimeUpdateR {
    OtherworldlyErr(__VMError),
}

impl<T> Monad for DebugtimeUpdateM<T> {}

impl<T> DebugtimeUpdateM<T> {
    pub(crate) fn result(self) -> DebugtimeUpdateM<__VMResult<T>> {
        match self {
            DebugtimeUpdateM::Ok(output) => DebugtimeUpdateM::Ok(Ok(output)),
            DebugtimeUpdateM::OtherworldlyErr(_) => todo!(),
        }
    }
}
impl<T> DebugtimeTakeChangeM<T> {
    pub fn result(self) -> DebugtimeTakeChangeM<__VMResult<T>> {
        match self {
            DebugtimeTakeChangeM::Ok(t) => DebugtimeTakeChangeM::Ok(Ok(t)),
            DebugtimeTakeChangeM::OtherworldlyErr(e) => DebugtimeTakeChangeM::Ok(Err(e)),
        }
    }
}

impl Debugtime {
    pub(crate) fn update(&mut self) -> DebugtimeUpdateM<()> {
        match self.try_update().result()? {
            Ok(()) => DebugtimeUpdateM::Ok(()),
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

    fn try_update(&mut self) -> DebugtimeUpdateM<()> {
        self.update_figure_canvases()?;
        self.update_figure_controls()?;
        self.update_trace_stalks()?;
        self.update_trace_statss()
    }
}

impl<T> FromResidual<Result<std::convert::Infallible, __VMError>> for DebugtimeUpdateM<T> {
    fn from_residual(_residual: Result<std::convert::Infallible, __VMError>) -> Self {
        todo!()
    }
}

// todo: refine this
impl<T> FromResidual<ServerTraceStateUpdateR<T>> for DebugtimeUpdateM<T> {
    fn from_residual(_residual: ServerTraceStateUpdateR<T>) -> Self {
        todo!()
    }
}

impl<T> FromResidual<DebugtimeUpdateR> for DebugtimeUpdateM<T> {
    fn from_residual(_residual: DebugtimeUpdateR) -> Self {
        todo!()
    }
}

impl<T> FromResidual<TrackableMakeChangeR<TrackableMap<TraceStatsKey, Option<TraceStats>>>>
    for DebugtimeUpdateM<T>
{
    fn from_residual(
        _residual: TrackableMakeChangeR<TrackableMap<TraceStatsKey, Option<TraceStats>>>,
    ) -> Self {
        todo!()
    }
}

impl<T> FromResidual<TrackableMakeChangeR<TrackableVec<TraceNode>>> for DebugtimeUpdateM<T> {
    fn from_residual(_residual: TrackableMakeChangeR<TrackableVec<TraceNode>>) -> Self {
        todo!()
    }
}

impl<T> FromResidual<TrackableMakeChangeR<TrackableAtom<Presentation>>> for DebugtimeUpdateM<T> {
    fn from_residual(_residual: TrackableMakeChangeR<TrackableAtom<Presentation>>) -> Self {
        todo!()
    }
}

impl<T> FromResidual<TrackableMakeChangeR<DebugtimeState>> for DebugtimeUpdateM<T> {
    fn from_residual(_residual: TrackableMakeChangeR<DebugtimeState>) -> Self {
        todo!()
    }
}
impl<T>
    FromResidual<
        TrackableMakeChangeR<TrackableMap<GenericFigureCanvasKey, GenericFigureCanvasData>>,
    > for DebugtimeUpdateM<T>
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
    > for DebugtimeUpdateM<T>
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
    for DebugtimeUpdateM<T>
{
    fn from_residual(
        _residual: TrackableMakeChangeR<TrackableMap<FigureControlKey, FigureControlData>>,
    ) -> Self {
        todo!()
    }
}

impl<T> std::ops::Try for DebugtimeUpdateM<T> {
    type Output = T;

    type Residual = DebugtimeUpdateR;

    fn from_output(output: Self::Output) -> Self {
        DebugtimeUpdateM::Ok(output)
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            DebugtimeUpdateM::Ok(cont) => std::ops::ControlFlow::Continue(cont),
            DebugtimeUpdateM::OtherworldlyErr(e) => {
                std::ops::ControlFlow::Break(DebugtimeUpdateR::OtherworldlyErr(e))
            }
        }
    }
}
