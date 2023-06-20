use super::*;

#[salsa::interned(db = EntityTreeDb, jar = EntityTreeJar, constructor = new_inner)]
pub struct TraitItemNodePath {
    maybe_ambiguous_path: MaybeAmbiguousPath<TraitItemPath>,
}

impl TraitItemNodePath {
    fn new(db: &dyn EntityTreeDb, registry: &mut EntityNodeRegistry, path: TraitItemPath) -> Self {
        Self::new_inner(db, registry.issue_maybe_ambiguous_path(path))
    }

    pub fn path(self, db: &dyn EntityTreeDb) -> Option<TraitItemPath> {
        self.maybe_ambiguous_path(db).unambiguous_path()
    }

    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        self.maybe_ambiguous_path(db).path.module_path(db)
    }

    pub fn item_kind(self, db: &dyn EntityTreeDb) -> TraitItemKind {
        self.maybe_ambiguous_path(db).path.item_kind(db)
    }

    pub fn node(self, db: &dyn EntityTreeDb) -> TraitItemNode {
        todo!()
    }
}

impl HasNodePath for TraitItemPath {
    type NodePath = TraitItemNodePath;

    fn node_path(self, db: &dyn EntityTreeDb) -> Self::NodePath {
        TraitItemNodePath::new_inner(db, MaybeAmbiguousPath::from_path(self))
    }
}

#[salsa::tracked(db = EntityTreeDb, jar = EntityTreeJar, constructor = new_inner)]
pub struct TraitItemNode {}
