use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct EntityPathMenu3 {
    parent: EntityPathMenu2,
}

impl EntityPathMenu3 {
    pub(crate) fn new(
        _db: &dyn EntityPathDb,
        _toolchain: Toolchain,
        menu2: EntityPathMenu2,
    ) -> Self {
        Self { parent: menu2 }
    }
}

impl std::ops::Deref for EntityPathMenu3 {
    type Target = EntityPathMenu2;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}
