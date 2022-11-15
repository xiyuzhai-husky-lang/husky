use husky_compiler::{CompileHuskyR, CompilerInstance};
use husky_debugtime::{DebugtimeStateChange, DevtimeHotReloadR};
use relative_path::RelativePathBuf;
use std::ops::FromResidual;

use crate::*;

#[must_use]
pub enum DebuggerHotReloadM {
    Ok(DebugtimeStateChange),
}

pub struct DebuggerHotReloadR;

impl std::ops::Try for DebuggerHotReloadM {
    type Output = DebugtimeStateChange;

    type Residual = DebuggerHotReloadR;

    fn from_output(_output: Self::Output) -> Self {
        todo!()
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            DebuggerHotReloadM::Ok(change) => std::ops::ControlFlow::Continue(change),
        }
    }
}

impl FromResidual<DebuggerHotReloadR> for DebuggerHotReloadM {
    fn from_residual(_residual: DebuggerHotReloadR) -> Self {
        todo!()
    }
}

impl FromResidual<CompileHuskyR> for DebuggerHotReloadM {
    fn from_residual(_residual: CompileHuskyR) -> Self {
        todo!()
    }
}

impl FromResidual<DevtimeHotReloadR> for DebuggerHotReloadM {
    fn from_residual(_residual: DevtimeHotReloadR) -> Self {
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
