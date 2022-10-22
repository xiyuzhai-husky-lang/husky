use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct EntityPathMenu1 {
    parent: EntityPathMenu0,
    // modules
    core_marker: EntityPathItd,
}

impl EntityPathMenu1 {
    pub(crate) fn new(db: &dyn EntityPathDb, menu0: &EntityPathMenu0) -> Self {
        todo!()
    }
}

impl std::ops::Deref for EntityPathMenu1 {
    type Target = EntityPathMenu0;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}
