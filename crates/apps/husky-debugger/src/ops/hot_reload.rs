use std::ops::FromResidual;

use husky_tracetime::TracetimeHotReloadR;

use crate::*;

pub enum DebuggerHotReloadM {
    Ok(InitData),
}

pub struct DebuggerHotReloadR;

impl std::ops::Try for DebuggerHotReloadM {
    type Output = InitData;

    type Residual = DebuggerHotReloadR;

    fn from_output(output: Self::Output) -> Self {
        todo!()
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            DebuggerHotReloadM::Ok(init_data) => std::ops::ControlFlow::Continue(init_data),
        }
    }
}

impl FromResidual<DebuggerHotReloadR> for DebuggerHotReloadM {
    fn from_residual(residual: DebuggerHotReloadR) -> Self {
        todo!()
    }
}

impl FromResidual<TracetimeHotReloadR> for DebuggerHotReloadM {
    fn from_residual(residual: TracetimeHotReloadR) -> Self {
        todo!()
    }
}

impl HuskyDebuggerInternal {
    pub(crate) fn hot_reload(&mut self) -> DebuggerHotReloadM {
        msg_once!("todo");
        DebuggerHotReloadM::Ok(self.tracetime.hot_reload()?)
    }
}
