use crate::*;
use husky_entity_kind::EntityKind;
use husky_print_utils::epin;
use monad::MonadT;
use std::time::Instant;

#[must_use]
pub enum HuskyDevtimeHotReloadM {
    Ok(HuskyDevtimeStateChange),
}

impl Monad for HuskyDevtimeHotReloadM {}

impl<T> MonadT<HuskyDevtimeUpdateM<T>> for HuskyDevtimeHotReloadM {}
impl<T> MonadT<HuskyDevtimeTakeChangeM<T>> for HuskyDevtimeHotReloadM {}
impl MonadT<HuskyRuntimeHotReloadM> for HuskyDevtimeHotReloadM {}

impl HuskyDevtime {
    pub fn hot_reload(&mut self) -> HuskyDevtimeHotReloadM {
        self.runtime.hot_reload()?;
        let old_state = self.clear()?;
        self.gen_root_traces();
        self.mimic_old_state(old_state);
        self.update()?;
        HuskyDevtimeHotReloadM::Ok(self.take_change()?)
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

    fn mimic_old_state(&mut self, mut old_state: HuskyDevtimeOldState) -> HuskyDevtimeUpdateM<()> {
        // order matters
        self.mimic_old_expansions(&mut old_state)?;
        old_state.fix();
        self.mimic_old_presentation(&old_state)
    }

    fn mimic_old_expansions(
        &mut self,
        old_state: &mut HuskyDevtimeOldState,
    ) -> HuskyDevtimeUpdateM<()> {
        self.mimic_old_expansions_dfs(0, old_state)
    }

    fn mimic_old_expansions_dfs(
        &mut self,
        start: usize,
        old_state: &mut HuskyDevtimeOldState,
    ) -> HuskyDevtimeUpdateM<()> {
        let end = self.state.trace_nodes.len();
        if start >= end {
            return HuskyDevtimeUpdateM::Ok(());
        }
        for idx in start..end {
            let trace_node = &self.state.trace_nodes[idx];
            if let Some(old_trace_node) = old_state.try_match_node(trace_node) {
                if old_trace_node.expanded() != trace_node.expanded() {
                    let new_trace_id = trace_node.trace().id();
                    self.state
                        .trace_nodes
                        .update_elem(idx, |node| node.toggle_expansion())?;
                    self.update_subtraces(new_trace_id)?
                }
            }
        }
        self.mimic_old_expansions_dfs(end, old_state)
    }

    fn mimic_old_presentation(
        &mut self,
        old_state: &HuskyDevtimeOldState,
    ) -> HuskyDevtimeUpdateM<()> {
        self.state
            .set_presentation(old_state.mimic_presentation(&self.state.trace_nodes));
        HuskyDevtimeUpdateM::Ok(())
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
    type Output = HuskyDevtimeStateChange;

    type Residual = DevtimeHotReloadR;

    fn from_output(output: Self::Output) -> Self {
        HuskyDevtimeHotReloadM::Ok(output)
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            HuskyDevtimeHotReloadM::Ok(change) => std::ops::ControlFlow::Continue(change),
        }
    }
}
