use husky_compiler::{CompileHuskyR, CompilerInstance};
use husky_debugtime::{DevtimeHotReloadR, HuskyDevtimeStateChange};
use relative_path::RelativePathBuf;
use std::ops::FromResidual;

use crate::*;

#[must_use]
pub enum DebuggerHotReloadM {
    Ok(HuskyDevtimeStateChange),
}

pub struct DebuggerHotReloadR;

impl std::ops::Try for DebuggerHotReloadM {
    type Output = HuskyDevtimeStateChange;

    type Residual = DebuggerHotReloadR;

    fn from_output(output: Self::Output) -> Self {
        todo!()
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            DebuggerHotReloadM::Ok(change) => std::ops::ControlFlow::Continue(change),
        }
    }
}

impl FromResidual<DebuggerHotReloadR> for DebuggerHotReloadM {
    fn from_residual(residual: DebuggerHotReloadR) -> Self {
        todo!()
    }
}

impl FromResidual<CompileHuskyR> for DebuggerHotReloadM {
    fn from_residual(residual: CompileHuskyR) -> Self {
        todo!()
    }
}

impl FromResidual<DevtimeHotReloadR> for DebuggerHotReloadM {
    fn from_residual(residual: DevtimeHotReloadR) -> Self {
        todo!()
    }
}

impl HuskyDebuggerInternal {
    fn compiler_instance(&self) -> CompilerInstance {
        CompilerInstance::new(RelativePathBuf::from_path(&self.config.package_dir).unwrap())
    }
    pub(crate) fn hot_reload(&mut self) -> DebuggerHotReloadM {
        self.compiler_instance().compile_all()?;
        DebuggerHotReloadM::Ok(self.devtime.hot_reload()?)
    }
}
