use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct EntityPathMenu2 {
    core_marker_sized: EntityPath,
    parent: EntityPathMenu1,
}

impl EntityPathMenu2 {
    pub(crate) fn new(db: &dyn EntityPathDb, menu1: EntityPathMenu1) -> Self {
        todo!()
        // Self {
        //     core_marker_sized: menu1.core_marker().child(db, "Sized"),
        //     parent: menu1,
        // }
    }
}

impl std::ops::Deref for EntityPathMenu2 {
    type Target = EntityPathMenu1;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}
