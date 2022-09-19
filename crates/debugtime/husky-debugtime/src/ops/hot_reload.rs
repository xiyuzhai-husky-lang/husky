use crate::*;

#[must_use]
pub enum DebugtimeHotReloadM {
    Ok(InitData),
}

impl Monad for DebugtimeHotReloadM {}

impl Debugtime {
    pub fn hot_reload(&mut self) -> DebugtimeHotReloadM {
        todo!();
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
    }
}

pub struct DebugtimeHotReloadR;

impl std::ops::FromResidual<DebugtimeHotReloadR> for DebugtimeHotReloadM {
    fn from_residual(residual: DebugtimeHotReloadR) -> Self {
        unreachable!()
    }
}

impl std::ops::Try for DebugtimeHotReloadM {
    type Output = InitData;

    type Residual = DebugtimeHotReloadR;

    fn from_output(output: Self::Output) -> Self {
        DebugtimeHotReloadM::Ok(output)
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            DebugtimeHotReloadM::Ok(init_data) => std::ops::ControlFlow::Continue(init_data),
        }
    }
}
