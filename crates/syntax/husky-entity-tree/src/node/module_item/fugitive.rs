use super::*;

#[salsa::interned(db = EntityTreeDb, jar = EntityTreeJar, constructor = new_inner)]
pub struct FugitiveNodePath {
    pub path: MaybeAmbiguous<FugitivePath>,
}

impl FugitiveNodePath {
    pub(super) fn new(
        db: &dyn EntityTreeDb,
        registry: &mut EntityNodeRegistry,
        path: FugitivePath,
    ) -> Self {
        Self::new_inner(db, registry.issue_maybe_ambiguous(path))
    }

    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        self.path(db).module_path(db)
    }
}

impl HasNodePath for FugitivePath {
    type NodePath = FugitiveNodePath;

    fn node_path(self, db: &dyn EntityTreeDb) -> Self::NodePath {
        FugitiveNodePath::new_inner(db, MaybeAmbiguous::from_path(self))
    }
}

impl From<FugitiveNodePath> for EntityNodePath {
    fn from(id: FugitiveNodePath) -> Self {
        EntityNodePath::ModuleItem(id.into())
    }
}
