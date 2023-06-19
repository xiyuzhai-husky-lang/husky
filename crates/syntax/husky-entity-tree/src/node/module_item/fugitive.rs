use super::*;

#[salsa::interned(db = EntityTreeDb, jar = EntityTreeJar, constructor = new_inner)]
pub struct FugitiveNodeId {
    pub maybe_ambiguous_path: MaybeAmbiguousPath<FugitivePath>,
}

impl FugitiveNodeId {
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

impl HasNodeId for FugitivePath {
    type NodeId = FugitiveNodeId;

    fn node_id(self, db: &dyn EntityTreeDb) -> Self::NodeId {
        FugitiveNodeId::new_inner(db, MaybeAmbiguousPath::from_path(self))
    }
}

impl From<FugitiveNodeId> for EntityNodeId {
    fn from(id: FugitiveNodeId) -> Self {
        EntityNodeId::ModuleItem(id.into())
    }
}
