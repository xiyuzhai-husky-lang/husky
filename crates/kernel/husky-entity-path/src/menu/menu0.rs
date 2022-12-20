use husky_absolute_path::AbsolutePathResult;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct EntityPathMenu0 {
    // library
    core: EntityPath,
    std: EntityPath,
}

impl EntityPathMenu0 {
    pub(crate) fn new(db: &dyn EntityPathDb, toolchain: Toolchain) -> EntityPathResult<Self> {
        Ok(Self {
            core: db.it_builtin_lib_path(toolchain, "core")?,
            std: db.it_builtin_lib_path(toolchain, "std")?,
        })
    }

    pub fn core(&self) -> EntityPath {
        self.core
    }

    pub fn std(&self) -> EntityPath {
        self.std
    }
}
