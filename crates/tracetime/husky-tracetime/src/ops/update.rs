use crate::*;
use husky_entity_kind::EntityKind;
use monad::Monad;
use std::{ops::FromResidual, time::Instant};

#[must_use]
pub enum TracetimeUpdateM<T> {
    Ok(T),
    OtherworldlyErr(__VMError),
}

pub enum TracetimeUpdateR {
    OtherworldlyErr(__VMError),
}

impl<T> Monad for TracetimeUpdateM<T> {}

impl<T> TracetimeUpdateM<T> {
    pub(crate) fn result(self) -> TracetimeUpdateM<__VMResult<T>> {
        todo!()
    }
}

impl Tracetime {
    pub(crate) fn update(&mut self) -> TracetimeUpdateM<()> {
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
        TracetimeUpdateM::Ok(())
    }

    fn clear(&mut self) -> TracetimeUpdateM<()> {
        // replace this with diff, try to make the trace tree look the same across code change
        self.state = Default::default();
        TracetimeUpdateM::Ok(())
    }

    fn update_root_traces(&mut self) -> TracetimeUpdateM<()> {
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
        TracetimeUpdateM::Ok(())
    }
}

impl<T> From<__VMResult<T>> for TracetimeUpdateM<T> {
    fn from(result: __VMResult<T>) -> Self {
        match result {
            Ok(cont) => TracetimeUpdateM::Ok(cont),
            Err(_) => todo!(),
        }
    }
}

impl<T> std::ops::Try for TracetimeUpdateM<T> {
    type Output = T;

    type Residual = TracetimeUpdateR;

    fn from_output(output: Self::Output) -> Self {
        TracetimeUpdateM::Ok(output)
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            TracetimeUpdateM::Ok(cont) => std::ops::ControlFlow::Continue(cont),
            TracetimeUpdateM::OtherworldlyErr(e) => {
                std::ops::ControlFlow::Break(TracetimeUpdateR::OtherworldlyErr(e))
            }
        }
    }
}

impl<T> FromResidual<TracetimeUpdateR> for TracetimeUpdateM<T> {
    fn from_residual(residual: TracetimeUpdateR) -> Self {
        todo!()
    }
}
