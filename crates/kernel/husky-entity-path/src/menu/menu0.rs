use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct EntityPathMenu0 {
    // library
    core: EntityPath,
    std: EntityPath,
}

impl EntityPathMenu0 {
    pub(crate) fn new(db: &dyn EntityPathDb, toolchain: Toolchain) -> Self {
        Self {
            core: db.it_builtin_lib_path(toolchain, "core").unwrap(),
            std: db.it_builtin_lib_path(toolchain, "std").unwrap(),
        }
    }

    pub fn core(&self) -> EntityPath {
        self.core
    }

    pub fn std(&self) -> EntityPath {
        self.std
    }
}
