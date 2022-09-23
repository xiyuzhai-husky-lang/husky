use crate::*;
use husky_entity_kind::EntityKind;
use monad::Monad;
use proj_like::{ProjAtom, ProjMakeChangeR, ProjMap};
use std::{ops::FromResidual, time::Instant};

#[must_use]
pub(crate) enum HuskyDebugtimeMakeChangeM<T> {
    Ok(T),
    OtherworldlyErr(__VMError),
}

pub enum HuskyDebugtimeMakeChangeR {
    OtherworldlyErr(__VMError),
}

#[must_use]
pub enum HuskyDebugtimeStageChangeM<T> {
    Ok(T),
    OtherworldlyErr(__VMError),
}

pub enum HuskyDebugtimeStageChangeR {
    OtherworldlyErr(__VMError),
}

impl<T> Monad for HuskyDebugtimeMakeChangeM<T> {}

impl<T> Monad for HuskyDebugtimeStageChangeM<T> {}

impl<T> HuskyDebugtimeMakeChangeM<T> {
    pub(crate) fn result(self) -> HuskyDebugtimeMakeChangeM<__VMResult<T>> {
        match self {
            HuskyDebugtimeMakeChangeM::Ok(output) => HuskyDebugtimeMakeChangeM::Ok(Ok(output)),
            HuskyDebugtimeMakeChangeM::OtherworldlyErr(_) => todo!(),
        }
    }
}
impl<T> HuskyDebugtimeStageChangeM<T> {
    pub fn result(self) -> HuskyDebugtimeStageChangeM<__VMResult<T>> {
        todo!()
    }
}

impl HuskyDebugtime {
    pub(crate) fn update(&mut self) -> HuskyDebugtimeMakeChangeM<()> {
        match self.try_update().result()? {
            Ok(()) => HuskyDebugtimeMakeChangeM::Ok(()),
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

    fn try_update(&mut self) -> HuskyDebugtimeMakeChangeM<()> {
        self.update_root_traces()?;
        self.update_figure_canvases()?;
        self.update_figure_controls()?;
        self.update_trace_stalks()?;
        self.update_trace_statss()
    }

    fn update_root_traces(&mut self) -> HuskyDebugtimeMakeChangeM<()> {
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
        HuskyDebugtimeMakeChangeM::Ok(())
    }
}

impl<T> FromResidual<HuskyDebugtimeMakeChangeR> for HuskyDebugtimeMakeChangeM<T> {
    fn from_residual(residual: HuskyDebugtimeMakeChangeR) -> Self {
        todo!()
    }
}

impl<T> FromResidual<ProjMakeChangeR<ProjAtom<Restriction>>> for HuskyDebugtimeMakeChangeM<T> {
    fn from_residual(residual: ProjMakeChangeR<ProjAtom<Restriction>>) -> Self {
        todo!()
    }
}

impl<T> FromResidual<ProjMakeChangeR<ProjMap<FigureCanvasKey, FigureCanvasData>>>
    for HuskyDebugtimeMakeChangeM<T>
{
    fn from_residual(
        residual: ProjMakeChangeR<ProjMap<FigureCanvasKey, FigureCanvasData>>,
    ) -> Self {
        todo!()
    }
}

impl<T> FromResidual<ProjMakeChangeR<ProjMap<FigureControlKey, FigureControlData>>>
    for HuskyDebugtimeMakeChangeM<T>
{
    fn from_residual(
        residual: ProjMakeChangeR<ProjMap<FigureControlKey, FigureControlData>>,
    ) -> Self {
        todo!()
    }
}

impl<T> std::ops::Try for HuskyDebugtimeMakeChangeM<T> {
    type Output = T;

    type Residual = HuskyDebugtimeMakeChangeR;

    fn from_output(output: Self::Output) -> Self {
        HuskyDebugtimeMakeChangeM::Ok(output)
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            HuskyDebugtimeMakeChangeM::Ok(cont) => std::ops::ControlFlow::Continue(cont),
            HuskyDebugtimeMakeChangeM::OtherworldlyErr(e) => {
                std::ops::ControlFlow::Break(HuskyDebugtimeMakeChangeR::OtherworldlyErr(e))
            }
        }
    }
}

impl<T> FromResidual<HuskyDebugtimeStageChangeR> for HuskyDebugtimeStageChangeM<T> {
    fn from_residual(residual: HuskyDebugtimeStageChangeR) -> Self {
        todo!()
    }
}

impl<T> FromResidual<HuskyDebugtimeMakeChangeR> for HuskyDebugtimeStageChangeM<T> {
    fn from_residual(residual: HuskyDebugtimeMakeChangeR) -> Self {
        todo!()
    }
}

impl<T> FromResidual<ProjMakeChangeR<ProjAtom<Restriction>>> for HuskyDebugtimeStageChangeM<T> {
    fn from_residual(residual: ProjMakeChangeR<ProjAtom<Restriction>>) -> Self {
        todo!()
    }
}

impl<T> std::ops::Try for HuskyDebugtimeStageChangeM<T> {
    type Output = T;

    type Residual = HuskyDebugtimeStageChangeR;

    fn from_output(output: Self::Output) -> Self {
        HuskyDebugtimeStageChangeM::Ok(output)
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            HuskyDebugtimeStageChangeM::Ok(cont) => std::ops::ControlFlow::Continue(cont),
            HuskyDebugtimeStageChangeM::OtherworldlyErr(e) => {
                std::ops::ControlFlow::Break(HuskyDebugtimeStageChangeR::OtherworldlyErr(e))
            }
        }
    }
}
