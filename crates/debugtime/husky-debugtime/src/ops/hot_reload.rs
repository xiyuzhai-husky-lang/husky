use crate::*;

#[must_use]
pub enum TracetimeHotReloadM {
    Ok(InitData),
}

impl Monad for TracetimeHotReloadM {}

impl Tracetime {
    pub fn hot_reload(&mut self) -> TracetimeHotReloadM {
        todo!();
        // let root_trace_ids = self.state.root_traces().to_vec();
        // // clear figure cache to reduce data transmission
        // self.state.figure_canvases.clear();
        // self.state.figure_controls.clear();
        // let figure_canvases = self.update_figure_canvases()?;
        // let figure_controls = self.update_figure_controls()?;
        // let pins = self.state.pins.clone();
        // let traces = self.all_trace_nodes();
        // TracetimeHotReloadM::Ok(InitData {
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

pub struct TracetimeHotReloadR;

impl std::ops::FromResidual<TracetimeHotReloadR> for TracetimeHotReloadM {
    fn from_residual(residual: TracetimeHotReloadR) -> Self {
        unreachable!()
    }
}

impl std::ops::Try for TracetimeHotReloadM {
    type Output = InitData;

    type Residual = TracetimeHotReloadR;

    fn from_output(output: Self::Output) -> Self {
        TracetimeHotReloadM::Ok(output)
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            TracetimeHotReloadM::Ok(init_data) => std::ops::ControlFlow::Continue(init_data),
        }
    }
}
