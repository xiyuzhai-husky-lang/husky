use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct EntityPathMenu2 {
    parent: EntityPathMenu1,
    core_marker_sized: EntityPathItd,
}

impl EntityPathMenu2 {
    pub(crate) fn new(db: &dyn EntityPathDb, menu1: EntityPathMenu1) -> Self {
        todo!()
    }
}

impl std::ops::Deref for EntityPathMenu2 {
    type Target = EntityPathMenu1;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}
