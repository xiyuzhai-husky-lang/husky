use crate::*;
use husky_entity_kind::EntityKind;
use monad::Monad;
use std::{ops::FromResidual, time::Instant};

#[must_use]
pub(crate) enum TracetimeUpdatingM<T> {
    Ok(T),
    OtherworldlyErr(__VMError),
}

pub enum TracetimeUpdatingR {
    OtherworldlyErr(__VMError),
}

#[must_use]
pub enum TracetimeUpdatedM<T> {
    Ok(T),
    OtherworldlyErr(__VMError),
}

pub enum TracetimeUpdatedR {
    OtherworldlyErr(__VMError),
}

impl<T> Monad for TracetimeUpdatingM<T> {}

impl<T> Monad for TracetimeUpdatedM<T> {}

impl<T> TracetimeUpdatingM<T> {
    pub(crate) fn result(self) -> TracetimeUpdatingM<__VMResult<T>> {
        match self {
            TracetimeUpdatingM::Ok(output) => TracetimeUpdatingM::Ok(Ok(output)),
            TracetimeUpdatingM::OtherworldlyErr(_) => todo!(),
        }
    }
}
impl<T> TracetimeUpdatedM<T> {
    pub fn result(self) -> TracetimeUpdatedM<__VMResult<T>> {
        todo!()
    }
}

impl Tracetime {
    pub(crate) fn updating(&mut self) -> TracetimeUpdatingM<()> {
        self.clear()?;
        // ad hoc
        match self.update_root_traces().result()? {
            Ok(()) => (),
            Err(e) => match e.variant() {
                __VMErrorVariant::Normal => todo!(),
                __VMErrorVariant::FromBatch { sample_id } => {
                    self.state.restriction.set_specific(SampleId(*sample_id));
                    self.update_trace_stalks();
                }
            },
        }
        TracetimeUpdatingM::Ok(())
    }

    fn clear(&mut self) -> TracetimeUpdatingM<()> {
        // replace this with diff, try to make the trace tree look the same across code change
        self.state = Default::default();
        TracetimeUpdatingM::Ok(())
    }

    fn update_root_traces(&mut self) -> TracetimeUpdatingM<()> {
        let target_entrance = self.comptime().target_entrance();
        let now = Instant::now();
        let main_feature_repr = self.runtime().main_feature_repr(target_entrance);
        println!(
            "{} milliseconds elapsed for computing main feature",
            now.elapsed().as_millis(),
        );
        let module = self.comptime().module(target_entrance).unwrap();
        let mut root_traces = vec![];
        // add input trace
        root_traces.push(self.new_trace(None, 0, TraceVariant::input(self.runtime())));
        // add module traces
        for (subentity_kind, subentity_route) in
            self.comptime().subentity_kinded_routes(module).iter()
        {
            match subentity_kind {
                EntityKind::Module => {
                    if self.comptime().module_contains_features(*subentity_route) {
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
        TracetimeUpdatingM::Ok(())
    }
}

// impl<T> From<__VMResult<T>> for TracetimeUpdatingM<T> {
//     fn from(result: __VMResult<T>) -> Self {
//         match result {
//             Ok(cont) => TracetimeUpdatingM::Ok(cont),
//             Err(_) => todo!(),
//         }
//     }
// }
impl<T> FromResidual<TracetimeUpdatingR> for TracetimeUpdatingM<T> {
    fn from_residual(residual: TracetimeUpdatingR) -> Self {
        todo!()
    }
}

impl<T> std::ops::Try for TracetimeUpdatingM<T> {
    type Output = T;

    type Residual = TracetimeUpdatingR;

    fn from_output(output: Self::Output) -> Self {
        TracetimeUpdatingM::Ok(output)
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            TracetimeUpdatingM::Ok(cont) => std::ops::ControlFlow::Continue(cont),
            TracetimeUpdatingM::OtherworldlyErr(e) => {
                std::ops::ControlFlow::Break(TracetimeUpdatingR::OtherworldlyErr(e))
            }
        }
    }
}

impl<T> FromResidual<TracetimeUpdatedR> for TracetimeUpdatedM<T> {
    fn from_residual(residual: TracetimeUpdatedR) -> Self {
        todo!()
    }
}

impl<T> FromResidual<TracetimeUpdatingR> for TracetimeUpdatedM<T> {
    fn from_residual(residual: TracetimeUpdatingR) -> Self {
        todo!()
    }
}

impl<T> std::ops::Try for TracetimeUpdatedM<T> {
    type Output = T;

    type Residual = TracetimeUpdatedR;

    fn from_output(output: Self::Output) -> Self {
        TracetimeUpdatedM::Ok(output)
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            TracetimeUpdatedM::Ok(cont) => std::ops::ControlFlow::Continue(cont),
            TracetimeUpdatedM::OtherworldlyErr(e) => {
                std::ops::ControlFlow::Break(TracetimeUpdatedR::OtherworldlyErr(e))
            }
        }
    }
}
