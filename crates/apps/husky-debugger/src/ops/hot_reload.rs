use std::ops::FromResidual;

use husky_compiler::{CompileHuskyR, CompilerInstance};
use husky_tracetime::HuskyTracetimeHotReloadR;
use relative_path::RelativePathBuf;

use crate::*;

#[must_use]
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

impl FromResidual<CompileHuskyR> for DebuggerHotReloadM {
    fn from_residual(residual: CompileHuskyR) -> Self {
        todo!()
    }
}

impl FromResidual<HuskyTracetimeHotReloadR> for DebuggerHotReloadM {
    fn from_residual(residual: HuskyTracetimeHotReloadR) -> Self {
        todo!()
    }
}

impl HuskyDebuggerInternal {
    fn compiler_instance(&self) -> CompilerInstance {
        CompilerInstance::new(RelativePathBuf::from_path(&self.config.package_dir).unwrap())
    }
    pub(crate) fn hot_reload(&mut self) -> DebuggerHotReloadM {
        self.compiler_instance().compile_all()?;
        DebuggerHotReloadM::Ok(self.tracetime.hot_reload()?)
    }
}
