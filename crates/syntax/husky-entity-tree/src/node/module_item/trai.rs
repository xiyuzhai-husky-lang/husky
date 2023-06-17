use super::*;

#[salsa::interned(db = EntityTreeDb, jar = EntityTreeJar, constructor = new_inner)]
pub struct TraitNodePath {
    pub maybe_ambiguous_path: MaybeAmbiguousPath<TraitPath>,
}

impl TraitNodePath {
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

impl HasNodePath for TraitPath {
    type NodePath = TraitNodePath;

    fn node_path(self, db: &dyn EntityTreeDb) -> Self::NodePath {
        TraitNodePath::new_inner(db, MaybeAmbiguousPath::from_path(self))
    }
}

impl From<TraitNodePath> for EntityNodePath {
    fn from(id: TraitNodePath) -> Self {
        EntityNodePath::ModuleItem(id.into())
    }
}
