use super::*;

#[salsa::interned(db = EntityTreeDb, jar = EntityTreeJar, constructor = new_inner)]
pub struct TraitNodePath {
    pub path: TraitPath,
}

impl TraitNodePath {
    pub(super) fn new(
        db: &dyn EntityTreeDb,
        registry: &mut EntityNodeRegistry,
        path: TraitPath,
    ) -> Self {
        Self::new_inner(db, path)
    }

    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        self.path(db).module_path(db)
    }
}

impl HasNodePath for TraitPath {
    type NodePath = TraitNodePath;

    fn node_path(self, db: &dyn EntityTreeDb) -> Self::NodePath {
        TraitNodePath::new_inner(db, self)
    }
}

impl From<TraitNodePath> for EntityNodePath {
    fn from(id: TraitNodePath) -> Self {
        EntityNodePath::ModuleItem(id.into())
    }
}
