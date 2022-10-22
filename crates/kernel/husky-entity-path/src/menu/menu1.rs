use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct EntityPathMenu1 {
    // modules
    core_marker: EntityPathItd,
    parent: EntityPathMenu0,
}

impl EntityPathMenu1 {
    pub(crate) fn new(db: &dyn EntityPathDb, menu0: EntityPathMenu0) -> Self {
        Self {
            core_marker: menu0.core().child(db, "marker"),
            parent: menu0,
        }
    }

    pub fn core_marker(&self) -> EntityPathItd {
        self.core_marker
    }
}

impl std::ops::Deref for EntityPathMenu1 {
    type Target = EntityPathMenu0;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}
