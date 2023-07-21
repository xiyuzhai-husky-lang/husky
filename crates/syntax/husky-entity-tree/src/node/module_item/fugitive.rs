use husky_entity_taxonomy::FugitiveKind;

use super::*;

#[salsa::interned(db = EntityTreeDb, jar = EntityTreeJar, constructor = new_inner)]
pub struct FugitiveSynNodePath {
    pub maybe_ambiguous_path: MaybeAmbiguousPath<FugitivePath>,
}

impl From<FugitiveSynNodePath> for EntitySynNodePath {
    fn from(id: FugitiveSynNodePath) -> Self {
        EntitySynNodePath::ModuleItem(id.into())
    }
}

impl HasSynNodePath for FugitivePath {
    type SynNodePath = FugitiveSynNodePath;

    fn node_path(self, db: &dyn EntityTreeDb) -> Self::SynNodePath {
        FugitiveSynNodePath::new_inner(db, MaybeAmbiguousPath::from_path(self))
    }
}

impl FugitiveSynNodePath {
    pub(super) fn new(
        db: &dyn EntityTreeDb,
        registry: &mut EntityNodeRegistry,
        path: FugitivePath,
    ) -> Self {
        Self::new_inner(db, registry.issue_maybe_ambiguous_path(path))
    }

    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        self.maybe_ambiguous_path(db).path.module_path(db)
    }

    pub fn ident(self, db: &dyn EntityTreeDb) -> Ident {
        self.maybe_ambiguous_path(db).path.ident(db)
    }

    pub fn fugitive_kind(self, db: &dyn EntityTreeDb) -> FugitiveKind {
        self.maybe_ambiguous_path(db).path.fugitive_kind(db)
    }

    pub fn node(self, db: &dyn EntityTreeDb) -> ModuleItemSynNode {
        fugitive_node(db, self)
    }
}

#[salsa::tracked(jar = EntityTreeJar)]
pub(crate) fn fugitive_node(
    db: &dyn EntityTreeDb,
    node_path: FugitiveSynNodePath,
) -> ModuleItemSynNode {
    let module_path = node_path.module_path(db);
    let entity_sheet = module_path.entity_tree_sheet(db).expect("valid file");
    match entity_sheet
        .major_entity_node(node_path.into())
        .expect("should be some")
    {
        EntitySynNode::ModuleItem(node) => node,
        _ => unreachable!(),
    }
}
