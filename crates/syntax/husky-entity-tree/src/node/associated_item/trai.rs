use super::*;

#[salsa::interned(db = EntityTreeDb, jar = EntityTreeJar)]
pub struct TraitItemNodeId {
    pub path: TraitItemPath,
}

impl TraitItemNodeId {
    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        self.path(db).module_path(db)
    }
}

#[salsa::tracked(db = EntityTreeDb, jar = EntityTreeJar, constructor = new_inner)]
pub struct TraitItemNode {}
