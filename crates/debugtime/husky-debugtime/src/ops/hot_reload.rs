use crate::*;
use monad::MonadT;

#[must_use]
pub enum HuskyDebugtimeHotReloadM {
    Ok(InitData),
}

impl Monad for HuskyDebugtimeHotReloadM {}

impl<T> MonadT<HuskyDebugtimeMakeChangeM<T>> for HuskyDebugtimeHotReloadM {}
impl<T> MonadT<HuskyDebugtimeStageChangeM<T>> for HuskyDebugtimeHotReloadM {}
impl MonadT<HuskyRuntimeHotReloadM> for HuskyDebugtimeHotReloadM {}

impl HuskyDebugtime {
    pub fn hot_reload(&mut self) -> HuskyDebugtimeHotReloadM {
        self.runtime.hot_reload()?;
        self.clear()?;
        self.update()?;
        HuskyDebugtimeHotReloadM::Ok(self.take_init_data()?)
    }

    pub fn take_init_data(&mut self) -> HuskyDebugtimeStageChangeM<InitData> {
        // let root_trace_ids = self.state.root_traces().to_vec();
        // // clear figure cache to reduce data transmission
        // self.state.figure_canvases.clear();
        // self.state.figure_controls.clear();
        // let figure_canvases = self.update_figure_canvases()?;
        // let figure_controls = self.update_figure_controls()?;
        // let pins = self.state.pins.clone();
        // let traces = self.all_trace_nodes();
        // DebugtimeHotReloadM::Ok(InitData {
        //     trace_init_data: TraceInitData {
        //         opt_active_trace_id: self.state.opt_active_trace_id,
        //         trace_nodes: traces,
        //         subtrace_ids_map: self
        //             .state
        //             .subtrace_ids_map
        //             .iter()
        //             .map(|(k, v)| (k.clone(), v.clone()))
        //             .collect(),
        //         trace_stalks: self
        //             .state
        //             .trace_stalks
        //             .iter()
        //             .map(|(k, v)| (k.clone(), v.clone()))
        //             .collect(),
        //         trace_statss: self
        //             .state
        //             .trace_statss
        //             .iter()
        //             .map(|(k, v)| (k.clone(), v.clone()))
        //             .collect(),
        //         root_trace_ids,
        //     },
        //     restriction: self.state.restriction.clone(),
        //     figure_canvases,
        //     figure_controls,
        //     pins,
        // })
        todo!()
    }
}

pub struct DebugtimeHotReloadR;

impl std::ops::FromResidual<DebugtimeHotReloadR> for HuskyDebugtimeHotReloadM {
    fn from_residual(residual: DebugtimeHotReloadR) -> Self {
        unreachable!()
    }
}

impl std::ops::FromResidual<HuskyDebugtimeMakeChangeR> for HuskyDebugtimeHotReloadM {
    fn from_residual(residual: HuskyDebugtimeMakeChangeR) -> Self {
        unreachable!()
    }
}

impl std::ops::FromResidual<HuskyDebugtimeStageChangeR> for HuskyDebugtimeHotReloadM {
    fn from_residual(residual: HuskyDebugtimeStageChangeR) -> Self {
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
