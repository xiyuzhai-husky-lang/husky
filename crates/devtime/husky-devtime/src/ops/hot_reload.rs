use crate::*;
use husky_entity_kind::EntityKind;
use monad::MonadT;
use std::time::Instant;

#[must_use]
pub enum HuskyDebugtimeHotReloadM {
    Ok(InitData),
}

impl Monad for HuskyDebugtimeHotReloadM {}

impl<T> MonadT<HuskyDebugtimeUpdateM<T>> for HuskyDebugtimeHotReloadM {}
impl<T> MonadT<HuskyDebugtimeTakeChangeM<T>> for HuskyDebugtimeHotReloadM {}
impl MonadT<HuskyRuntimeHotReloadM> for HuskyDebugtimeHotReloadM {}

impl HuskyDevtime {
    pub fn hot_reload(&mut self) -> HuskyDebugtimeHotReloadM {
        self.runtime.hot_reload()?;
        self.clear()?;
        self.gen_root_traces(); // ad hoc
        self.update()?;
        HuskyDebugtimeHotReloadM::Ok(self.take_init_data()?)
    }

    fn gen_root_traces(&mut self) {
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
    }

    pub fn take_init_data(&mut self) -> HuskyDebugtimeTakeChangeM<InitData> {
        // ignored
        let _staged_change = self.take_change()?;
        // ad hoc
        let init_data = InitData {
            restriction: self.restriction().clone(),
            trace_init_data: TraceInitData {
                trace_nodes: self.all_trace_nodes(),
                opt_active_trace_id: self.opt_active_trace_id(),
                subtrace_ids_map: self
                    .state
                    .subtrace_ids_map
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect(),
                trace_stalks: self
                    .state
                    .trace_stalks
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect(),
                trace_statss: self
                    .state
                    .trace_statss
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect(),
                root_trace_ids: self.root_traces(),
            },
            figure_canvases: self.state.figure_canvases.to_vec(),
            figure_controls: self.state.figure_controls.to_vec(),
            pins: self.state.pins.clone(),
        };
        HuskyDebugtimeTakeChangeM::Ok(init_data)
    }
}

pub struct DebugtimeHotReloadR;

impl std::ops::FromResidual<DebugtimeHotReloadR> for HuskyDebugtimeHotReloadM {
    fn from_residual(residual: DebugtimeHotReloadR) -> Self {
        unreachable!()
    }
}

impl std::ops::FromResidual<HuskyDebugtimeUpdateR> for HuskyDebugtimeHotReloadM {
    fn from_residual(residual: HuskyDebugtimeUpdateR) -> Self {
        unreachable!()
    }
}

impl std::ops::FromResidual<HuskyDebugtimeTakeChangeR> for HuskyDebugtimeHotReloadM {
    fn from_residual(residual: HuskyDebugtimeTakeChangeR) -> Self {
        unreachable!()
    }
}

impl std::ops::FromResidual<HuskyRuntimeHotReloadR> for HuskyDebugtimeHotReloadM {
    fn from_residual(residual: HuskyRuntimeHotReloadR) -> Self {
        todo!()
    }
}

impl std::ops::Try for HuskyDebugtimeHotReloadM {
    type Output = InitData;

    type Residual = DebugtimeHotReloadR;

    fn from_output(output: Self::Output) -> Self {
        HuskyDebugtimeHotReloadM::Ok(output)
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            HuskyDebugtimeHotReloadM::Ok(init_data) => std::ops::ControlFlow::Continue(init_data),
        }
    }
}
