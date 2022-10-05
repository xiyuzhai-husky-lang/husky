use crate::*;
use husky_entity_kind::EntityKind;
use monad::MonadT;
use std::time::Instant;

#[must_use]
pub enum HuskyDevtimeHotReloadM {
    Ok(InitData),
}

impl Monad for HuskyDevtimeHotReloadM {}

impl<T> MonadT<HuskyDevtimeUpdateM<T>> for HuskyDevtimeHotReloadM {}
impl<T> MonadT<HuskyDevtimeTakeChangeM<T>> for HuskyDevtimeHotReloadM {}
impl MonadT<HuskyRuntimeHotReloadM> for HuskyDevtimeHotReloadM {}

impl HuskyDevtime {
    pub fn hot_reload(&mut self) -> HuskyDevtimeHotReloadM {
        self.runtime.hot_reload()?;
        self.clear()?;
        self.gen_root_traces(); // ad hoc
        self.update()?;
        HuskyDevtimeHotReloadM::Ok(self.take_init_data()?)
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

    pub fn take_init_data(&mut self) -> HuskyDevtimeTakeChangeM<InitData> {
        // ignored
        let _staged_change = self.take_change()?;
        // ad hoc
        let init_data = InitData {
            presentation: self.presentation().clone(),
            trace_init_data: TraceInitData {
                trace_nodes: self.all_trace_nodes(),
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
        };
        HuskyDevtimeTakeChangeM::Ok(init_data)
    }
}

pub struct DevtimeHotReloadR;

impl std::ops::FromResidual<DevtimeHotReloadR> for HuskyDevtimeHotReloadM {
    fn from_residual(residual: DevtimeHotReloadR) -> Self {
        unreachable!()
    }
}

impl std::ops::FromResidual<HuskyDevtimeUpdateR> for HuskyDevtimeHotReloadM {
    fn from_residual(residual: HuskyDevtimeUpdateR) -> Self {
        unreachable!()
    }
}

impl std::ops::FromResidual<HuskyDevtimeTakeChangeR> for HuskyDevtimeHotReloadM {
    fn from_residual(residual: HuskyDevtimeTakeChangeR) -> Self {
        unreachable!()
    }
}

impl std::ops::FromResidual<HuskyRuntimeHotReloadR> for HuskyDevtimeHotReloadM {
    fn from_residual(residual: HuskyRuntimeHotReloadR) -> Self {
        todo!()
    }
}

impl std::ops::Try for HuskyDevtimeHotReloadM {
    type Output = InitData;

    type Residual = DevtimeHotReloadR;

    fn from_output(output: Self::Output) -> Self {
        HuskyDevtimeHotReloadM::Ok(output)
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            HuskyDevtimeHotReloadM::Ok(init_data) => std::ops::ControlFlow::Continue(init_data),
        }
    }
}
