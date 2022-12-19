use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct EntityPathMenu1 {
    // modules
    core_marker: EntityPath,
    core_basic: EntityPath,
    core_num: EntityPath,
    core_ops: EntityPath,
    core_prelude: EntityPath,
    parent: EntityPathMenu0,
}

impl EntityPathMenu1 {
    pub(crate) fn new(db: &dyn EntityPathDb, toolchain: Toolchain, menu0: EntityPathMenu0) -> Self {
        Self {
            core_marker: menu0.core().child(db, "marker").unwrap(),
            core_basic: menu0.core().child(db, "basic").unwrap(),
            core_num: menu0.core().child(db, "num").unwrap(),
            core_ops: menu0.core().child(db, "ops").unwrap(),
            core_prelude: menu0.core().child(db, "prelude").unwrap(),
            parent: menu0,
        }
    }

    pub fn core_basic(&self) -> EntityPath {
        self.core_basic
    }

    pub fn core_marker(&self) -> EntityPath {
        self.core_marker
    }

    pub fn core_num(&self) -> EntityPath {
        self.core_num
    }

    pub fn core_ops(&self) -> EntityPath {
        self.core_ops
    }
}

impl std::ops::Deref for EntityPathMenu1 {
    type Target = EntityPathMenu0;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}
