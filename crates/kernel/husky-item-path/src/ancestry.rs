use crate::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityPathDb)]
pub struct EntityAncestry {
    crate_path: CratePath,
    modules: Vec<ModulePath>,
    entities: Vec<ItemPath>,
}

impl EntityAncestry {
    pub fn crate_path(&self) -> CratePath {
        self.crate_path
    }

    pub fn modules(&self) -> &[ModulePath] {
        self.modules.as_ref()
    }

    pub fn entities(&self) -> &[ItemPath] {
        self.entities.as_ref()
    }
}
