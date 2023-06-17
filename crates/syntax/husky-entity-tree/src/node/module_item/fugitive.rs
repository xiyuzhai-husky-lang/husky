use super::*;

#[salsa::interned(db = EntityTreeDb, jar = EntityTreeJar, constructor = new_inner)]
pub struct FugitiveNodePath {
    pub maybe_ambiguous_path: MaybeAmbiguousPath<FugitivePath>,
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
        self.maybe_ambiguous_path(db).path.module_path(db)
    }

    pub fn node(self, db: &dyn EntityTreeDb) -> ModuleItemNode {
        todo!()
    }
}

impl HasNodePath for FugitivePath {
    type NodePath = FugitiveNodePath;

    fn node_path(self, db: &dyn EntityTreeDb) -> Self::NodePath {
        FugitiveNodePath::new_inner(db, MaybeAmbiguousPath::from_path(self))
    }
}

impl From<FugitiveNodePath> for EntityNodePath {
    fn from(id: FugitiveNodePath) -> Self {
        EntityNodePath::ModuleItem(id.into())
    }
}
