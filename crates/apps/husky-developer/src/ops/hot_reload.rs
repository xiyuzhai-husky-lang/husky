use husky_compiler::{CompileHuskyR, CompilerInstance};
use husky_debugtime::{DevtimeHotReloadR, DevtimeStateChange};
use relative_path::RelativePathBuf;
use std::ops::FromResidual;

use crate::*;

impl HuskyDebuggerInternal {
    fn compiler_instance(&self) -> CompilerInstance {
        CompilerInstance::new(RelativePathBuf::from_path(&self.config.package_dir).unwrap())
    }
    pub(crate) fn hot_reload(&mut self) -> DebuggerHotReloadM {
        self.compiler_instance().compile_all()?;
        self.devtime.hot_reload()
    }
}
