use crate::*;
use husky_entity_kind::EntityKind;
use monad::Monad;
use proj_like::{ProjAtom, ProjMakeChangeR, ProjMap};
use std::{ops::FromResidual, time::Instant};

#[must_use]
pub(crate) enum DebugtimeMakeChangeM<T> {
    Ok(T),
    OtherworldlyErr(__VMError),
}

pub enum DebugtimeMakeChangeR {
    OtherworldlyErr(__VMError),
}

#[must_use]
pub enum DebugtimeStageChangeM<T> {
    Ok(T),
    OtherworldlyErr(__VMError),
}

pub enum DebugtimeUpdatedR {
    OtherworldlyErr(__VMError),
}

impl<T> Monad for DebugtimeMakeChangeM<T> {}

impl<T> Monad for DebugtimeStageChangeM<T> {}

impl<T> DebugtimeMakeChangeM<T> {
    pub(crate) fn result(self) -> DebugtimeMakeChangeM<__VMResult<T>> {
        match self {
            DebugtimeMakeChangeM::Ok(output) => DebugtimeMakeChangeM::Ok(Ok(output)),
            DebugtimeMakeChangeM::OtherworldlyErr(_) => todo!(),
        }
    }
}
impl<T> DebugtimeStageChangeM<T> {
    pub fn result(self) -> DebugtimeStageChangeM<__VMResult<T>> {
        todo!()
    }
}

impl HuskyDebugtime {
    pub(crate) fn update(&mut self) -> DebugtimeMakeChangeM<()> {
        match self.try_update().result()? {
            Ok(()) => DebugtimeMakeChangeM::Ok(()),
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

    fn try_update(&mut self) -> DebugtimeMakeChangeM<()> {
        self.update_root_traces()?;
        self.update_figure_canvases()?;
        self.update_figure_controls()?;
        self.update_trace_stalks()?;
        self.update_trace_statss()
    }

    fn update_root_traces(&mut self) -> DebugtimeMakeChangeM<()> {
        let target_entrance = self.runtime().target_entrance();
        let now = Instant::now();
        let main_feature_repr = self.runtime().main_feature_repr(target_entrance);
        println!(
            "{} milliseconds elapsed for computing main feature",
            now.elapsed().as_millis(),
        );
        let module = self.runtime().module(target_entrance).unwrap();
        let mut root_traces = vec![];
        // add input trace
        root_traces.push(self.new_trace(None, 0, TraceVariant::input(self.runtime())));
        // add module traces
        for (subentity_kind, subentity_route) in
            self.runtime().subentity_kinded_routes(module).iter()
        {
            match subentity_kind {
                EntityKind::Module => {
                    if self.runtime().module_contains_features(*subentity_route) {
                        root_traces.push(self.new_trace(
                            None,
                            0,
                            TraceVariant::Module {
                                route: *subentity_route,
                                file: target_entrance,
                                range: Default::default(),
                            },
                        ))
                    }
                }
                EntityKind::Feature => {
                    let repr = self.runtime().entity_feature_repr(*subentity_route);
                    root_traces.push(self.new_trace(
                        None,
                        0,
                        TraceVariant::EntityFeature {
                            route: *subentity_route,
                            repr,
                        },
                    ))
                }
                _ => (),
            }
        }
        // add main trace
        root_traces.push(self.new_trace(None, 0, TraceVariant::Main(main_feature_repr)));
        self.state.set_root_traces(root_traces);
        self.update_trace_statss()?;
        DebugtimeMakeChangeM::Ok(())
    }
}

impl<T> FromResidual<DebugtimeMakeChangeR> for DebugtimeMakeChangeM<T> {
    fn from_residual(residual: DebugtimeMakeChangeR) -> Self {
        todo!()
    }
}

impl<T> FromResidual<ProjMakeChangeR<ProjAtom<Restriction>>> for DebugtimeMakeChangeM<T> {
    fn from_residual(residual: ProjMakeChangeR<ProjAtom<Restriction>>) -> Self {
        todo!()
    }
}

impl<T> FromResidual<ProjMakeChangeR<ProjMap<FigureCanvasKey, FigureCanvasData>>>
    for DebugtimeMakeChangeM<T>
{
    fn from_residual(
        residual: ProjMakeChangeR<ProjMap<FigureCanvasKey, FigureCanvasData>>,
    ) -> Self {
        todo!()
    }
}

impl<T> FromResidual<ProjMakeChangeR<ProjMap<FigureControlKey, FigureControlData>>>
    for DebugtimeMakeChangeM<T>
{
    fn from_residual(
        residual: ProjMakeChangeR<ProjMap<FigureControlKey, FigureControlData>>,
    ) -> Self {
        todo!()
    }
}

impl<T> std::ops::Try for DebugtimeMakeChangeM<T> {
    type Output = T;

    type Residual = DebugtimeMakeChangeR;

    fn from_output(output: Self::Output) -> Self {
        DebugtimeMakeChangeM::Ok(output)
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            DebugtimeMakeChangeM::Ok(cont) => std::ops::ControlFlow::Continue(cont),
            DebugtimeMakeChangeM::OtherworldlyErr(e) => {
                std::ops::ControlFlow::Break(DebugtimeMakeChangeR::OtherworldlyErr(e))
            }
        }
    }
}

impl<T> FromResidual<DebugtimeUpdatedR> for DebugtimeStageChangeM<T> {
    fn from_residual(residual: DebugtimeUpdatedR) -> Self {
        todo!()
    }
}

impl<T> FromResidual<DebugtimeMakeChangeR> for DebugtimeStageChangeM<T> {
    fn from_residual(residual: DebugtimeMakeChangeR) -> Self {
        todo!()
    }
}

impl<T> FromResidual<ProjMakeChangeR<ProjAtom<Restriction>>> for DebugtimeStageChangeM<T> {
    fn from_residual(residual: ProjMakeChangeR<ProjAtom<Restriction>>) -> Self {
        todo!()
    }
}

impl<T> std::ops::Try for DebugtimeStageChangeM<T> {
    type Output = T;

    type Residual = DebugtimeUpdatedR;

    fn from_output(output: Self::Output) -> Self {
        DebugtimeStageChangeM::Ok(output)
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            DebugtimeStageChangeM::Ok(cont) => std::ops::ControlFlow::Continue(cont),
            DebugtimeStageChangeM::OtherworldlyErr(e) => {
                std::ops::ControlFlow::Break(DebugtimeUpdatedR::OtherworldlyErr(e))
            }
        }
    }
}
