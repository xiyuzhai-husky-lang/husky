use crate::*;
use husky_entity_kind::EntityKind;
use monad::Monad;
use std::{ops::FromResidual, time::Instant};

#[must_use]
pub(crate) enum DebugtimeUpdatingM<T> {
    Ok(T),
    OtherworldlyErr(__VMError),
}

pub enum DebugtimeUpdatingR {
    OtherworldlyErr(__VMError),
}

#[must_use]
pub enum DebugtimeUpdatedM<T> {
    Ok(T),
    OtherworldlyErr(__VMError),
}

pub enum DebugtimeUpdatedR {
    OtherworldlyErr(__VMError),
}

impl<T> Monad for DebugtimeUpdatingM<T> {}

impl<T> Monad for DebugtimeUpdatedM<T> {}

impl<T> DebugtimeUpdatingM<T> {
    pub(crate) fn result(self) -> DebugtimeUpdatingM<__VMResult<T>> {
        match self {
            DebugtimeUpdatingM::Ok(output) => DebugtimeUpdatingM::Ok(Ok(output)),
            DebugtimeUpdatingM::OtherworldlyErr(_) => todo!(),
        }
    }
}
impl<T> DebugtimeUpdatedM<T> {
    pub fn result(self) -> DebugtimeUpdatedM<__VMResult<T>> {
        todo!()
    }
}

impl Debugtime {
    pub(crate) fn updating(&mut self) -> DebugtimeUpdatingM<()> {
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
        DebugtimeUpdatingM::Ok(())
    }

    fn clear(&mut self) -> DebugtimeUpdatingM<()> {
        // replace this with diff, try to make the trace tree look the same across code change
        self.state = Default::default();
        DebugtimeUpdatingM::Ok(())
    }

    fn update_root_traces(&mut self) -> DebugtimeUpdatingM<()> {
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
        DebugtimeUpdatingM::Ok(())
    }
}

// impl<T> From<__VMResult<T>> for DebugtimeUpdatingM<T> {
//     fn from(result: __VMResult<T>) -> Self {
//         match result {
//             Ok(cont) => DebugtimeUpdatingM::Ok(cont),
//             Err(_) => todo!(),
//         }
//     }
// }
impl<T> FromResidual<DebugtimeUpdatingR> for DebugtimeUpdatingM<T> {
    fn from_residual(residual: DebugtimeUpdatingR) -> Self {
        todo!()
    }
}

impl<T> std::ops::Try for DebugtimeUpdatingM<T> {
    type Output = T;

    type Residual = DebugtimeUpdatingR;

    fn from_output(output: Self::Output) -> Self {
        DebugtimeUpdatingM::Ok(output)
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            DebugtimeUpdatingM::Ok(cont) => std::ops::ControlFlow::Continue(cont),
            DebugtimeUpdatingM::OtherworldlyErr(e) => {
                std::ops::ControlFlow::Break(DebugtimeUpdatingR::OtherworldlyErr(e))
            }
        }
    }
}

impl<T> FromResidual<DebugtimeUpdatedR> for DebugtimeUpdatedM<T> {
    fn from_residual(residual: DebugtimeUpdatedR) -> Self {
        todo!()
    }
}

impl<T> FromResidual<DebugtimeUpdatingR> for DebugtimeUpdatedM<T> {
    fn from_residual(residual: DebugtimeUpdatingR) -> Self {
        todo!()
    }
}

impl<T> std::ops::Try for DebugtimeUpdatedM<T> {
    type Output = T;

    type Residual = DebugtimeUpdatedR;

    fn from_output(output: Self::Output) -> Self {
        DebugtimeUpdatedM::Ok(output)
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            DebugtimeUpdatedM::Ok(cont) => std::ops::ControlFlow::Continue(cont),
            DebugtimeUpdatedM::OtherworldlyErr(e) => {
                std::ops::ControlFlow::Break(DebugtimeUpdatedR::OtherworldlyErr(e))
            }
        }
    }
}
