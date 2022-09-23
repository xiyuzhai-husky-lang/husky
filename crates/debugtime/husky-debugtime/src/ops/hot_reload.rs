use crate::*;
use monad::MonadT;

#[must_use]
pub enum HuskyDebugtimeHotReloadM {
    Ok(InitData),
}

impl Monad for HuskyDebugtimeHotReloadM {}

impl<T> MonadT<HuskyDebugtimeUpdateM<T>> for HuskyDebugtimeHotReloadM {}
impl<T> MonadT<HuskyDebugtimeTakeChangeM<T>> for HuskyDebugtimeHotReloadM {}
impl MonadT<HuskyRuntimeHotReloadM> for HuskyDebugtimeHotReloadM {}

impl HuskyDebugtime {
    pub fn hot_reload(&mut self) -> HuskyDebugtimeHotReloadM {
        self.runtime.hot_reload()?;
        self.clear()?;
        self.update()?;
        HuskyDebugtimeHotReloadM::Ok(self.take_init_data()?)
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
