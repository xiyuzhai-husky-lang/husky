use crate::*;
use husky_entity_kind::EntityKind;
use monad::Monad;
use std::{ops::FromResidual, time::Instant};
use trackable::{TrackableAtom, TrackableMakeChangeR, TrackableMap};

#[must_use]
pub(crate) enum HuskyDebugtimeUpdateM<T> {
    Ok(T),
    OtherworldlyErr(__VMError),
}

pub enum HuskyDebugtimeUpdateR {
    OtherworldlyErr(__VMError),
}

impl<T> Monad for HuskyDebugtimeUpdateM<T> {}

impl<T> HuskyDebugtimeUpdateM<T> {
    pub(crate) fn result(self) -> HuskyDebugtimeUpdateM<__VMResult<T>> {
        match self {
            HuskyDebugtimeUpdateM::Ok(output) => HuskyDebugtimeUpdateM::Ok(Ok(output)),
            HuskyDebugtimeUpdateM::OtherworldlyErr(_) => todo!(),
        }
    }
}
impl<T> HuskyDebugtimeTakeChangeM<T> {
    pub fn result(self) -> HuskyDebugtimeTakeChangeM<__VMResult<T>> {
        match self {
            HuskyDebugtimeTakeChangeM::Ok(t) => HuskyDebugtimeTakeChangeM::Ok(Ok(t)),
            HuskyDebugtimeTakeChangeM::OtherworldlyErr(e) => HuskyDebugtimeTakeChangeM::Ok(Err(e)),
        }
    }
}

impl HuskyDebugtime {
    pub(crate) fn update(&mut self) -> HuskyDebugtimeUpdateM<()> {
        match self.try_update().result()? {
            Ok(()) => HuskyDebugtimeUpdateM::Ok(()),
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

    fn try_update(&mut self) -> HuskyDebugtimeUpdateM<()> {
        self.update_root_traces()?;
        self.update_figure_canvases()?;
        self.update_figure_controls()?;
        self.update_trace_stalks()?;
        self.update_trace_statss()
    }

    fn update_root_traces(&mut self) -> HuskyDebugtimeUpdateM<()> {
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
        HuskyDebugtimeUpdateM::Ok(())
    }
}

impl<T> FromResidual<Result<std::convert::Infallible, __VMError>> for HuskyDebugtimeUpdateM<T> {
    fn from_residual(residual: Result<std::convert::Infallible, __VMError>) -> Self {
        todo!()
    }
}

impl<T> FromResidual<HuskyDebugtimeUpdateR> for HuskyDebugtimeUpdateM<T> {
    fn from_residual(residual: HuskyDebugtimeUpdateR) -> Self {
        todo!()
    }
}

impl<T> FromResidual<TrackableMakeChangeR<TrackableAtom<Restriction>>>
    for HuskyDebugtimeUpdateM<T>
{
    fn from_residual(residual: TrackableMakeChangeR<TrackableAtom<Restriction>>) -> Self {
        todo!()
    }
}

impl<T> FromResidual<TrackableMakeChangeR<TrackableMap<FigureCanvasKey, FigureCanvasData>>>
    for HuskyDebugtimeUpdateM<T>
{
    fn from_residual(
        residual: TrackableMakeChangeR<TrackableMap<FigureCanvasKey, FigureCanvasData>>,
    ) -> Self {
        todo!()
    }
}

impl<T> FromResidual<TrackableMakeChangeR<TrackableMap<FigureControlKey, FigureControlData>>>
    for HuskyDebugtimeUpdateM<T>
{
    fn from_residual(
        residual: TrackableMakeChangeR<TrackableMap<FigureControlKey, FigureControlData>>,
    ) -> Self {
        todo!()
    }
}

impl<T> std::ops::Try for HuskyDebugtimeUpdateM<T> {
    type Output = T;

    type Residual = HuskyDebugtimeUpdateR;

    fn from_output(output: Self::Output) -> Self {
        HuskyDebugtimeUpdateM::Ok(output)
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            HuskyDebugtimeUpdateM::Ok(cont) => std::ops::ControlFlow::Continue(cont),
            HuskyDebugtimeUpdateM::OtherworldlyErr(e) => {
                std::ops::ControlFlow::Break(HuskyDebugtimeUpdateR::OtherworldlyErr(e))
            }
        }
    }
}
