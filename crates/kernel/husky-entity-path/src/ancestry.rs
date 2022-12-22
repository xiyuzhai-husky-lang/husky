use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct EntityAncestry {
    crate_path: CratePath,
    modules: Vec<ModulePath>,
    entities: Vec<EntityPath>,
}

impl EntityAncestry {
    pub fn crate_path(&self) -> CratePath {
        self.crate_path
    }

    pub fn modules(&self) -> &[ModulePath] {
        self.modules.as_ref()
    }

    pub fn entities(&self) -> &[EntityPath] {
        self.entities.as_ref()
    }
}

impl salsa::DebugWithDb<dyn EntityPathDb + '_> for EntityAncestry {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn EntityPathDb,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        todo!()
    }
}

// #[salsa::tracked(jar = EntityPathJar, return_ref)]
// pub(crate) fn entity_module_ancestry(db: &dyn EntityPathDb, path: EntityPath) -> EntityAncestry {
//     todo!()
// }
