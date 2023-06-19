use super::*;

#[salsa::interned(db = EntityTreeDb, jar = EntityTreeJar, constructor = new_inner)]
pub struct TraitNodeId {
    pub maybe_ambiguous_path: MaybeAmbiguousPath<TraitPath>,
}

impl TraitNodeId {
    pub(super) fn new(
        db: &dyn EntityTreeDb,
        registry: &mut EntityNodeRegistry,
        path: TraitPath,
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

impl HasNodeId for TraitPath {
    type NodeId = TraitNodeId;

    fn node_id(self, db: &dyn EntityTreeDb) -> Self::NodeId {
        TraitNodeId::new_inner(db, MaybeAmbiguousPath::from_path(self))
    }
}

impl From<TraitNodeId> for EntityNodeId {
    fn from(id: TraitNodeId) -> Self {
        EntityNodeId::ModuleItem(id.into())
    }
}
