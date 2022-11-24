use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct EntityPathMenu1 {
    // modules
    core_marker: EntityPath,
    core_num: EntityPath,
    core_prelude: EntityPath,
    parent: EntityPathMenu0,
}

impl EntityPathMenu1 {
    pub(crate) fn new(db: &dyn EntityPathDb, menu0: EntityPathMenu0) -> Self {
        Self {
            core_marker: menu0.core().child(db, "marker"),
            core_num: menu0.core().child(db, "num"),
            core_prelude: menu0.core().child(db, "prelude"),
            parent: menu0,
        }
    }

    pub fn core_marker(&self) -> EntityPath {
        self.core_marker
    }

    pub fn core_num(&self) -> EntityPath {
        self.core_num
    }
}

impl std::ops::Deref for EntityPathMenu1 {
    type Target = EntityPathMenu0;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}
