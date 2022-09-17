use crate::*;

#[must_use]
pub enum HuskyTracetimeHotReloadM {
    Ok(InitData),
}

impl Monad for HuskyTracetimeHotReloadM {}

impl HuskyTracetime {
    pub fn hot_reload(&mut self) -> HuskyTracetimeHotReloadM {
        todo!();
        let root_trace_ids = self.root_trace_ids.clone();
        // clear figure cache to reduce data transmission
        self.figure_canvases.clear();
        self.figure_controls.clear();
        let figure_canvases = self.update_figure_canvases().expect("todo");
        let figure_controls = self.update_figure_controls().expect("todo");
        let pins = self.pins.clone();
        let traces = self.all_trace_nodes();
        HuskyTracetimeHotReloadM::Ok(InitData {
            trace_init_data: TraceInitData {
                opt_active_trace_id: self.opt_active_trace_id,
                trace_nodes: traces,
                subtrace_ids_map: self
                    .subtrace_ids_map
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect(),
                trace_stalks: self
                    .trace_stalks
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect(),
                trace_statss: self
                    .trace_statss
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect(),
                root_trace_ids,
            },
            restriction: self.restriction.clone(),
            figure_canvases,
            figure_controls,
            pins,
        })
    }
}

pub struct TracetimeHotReloadR;

impl std::ops::FromResidual<TracetimeHotReloadR> for HuskyTracetimeHotReloadM {
    fn from_residual(residual: TracetimeHotReloadR) -> Self {
        unreachable!()
    }
}

impl std::ops::Try for HuskyTracetimeHotReloadM {
    type Output = InitData;

    type Residual = TracetimeHotReloadR;

    fn from_output(output: Self::Output) -> Self {
        HuskyTracetimeHotReloadM::Ok(output)
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            HuskyTracetimeHotReloadM::Ok(init_data) => std::ops::ControlFlow::Continue(init_data),
        }
    }
}
