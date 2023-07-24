use husky_entity_taxonomy::FugitiveKind;

use super::*;

#[salsa::interned(db = EntitySynTreeDb, jar = EntitySynTreeJar, constructor = new_inner)]
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

    fn syn_node_path(self, db: &dyn EntitySynTreeDb) -> Self::SynNodePath {
        FugitiveSynNodePath::new_inner(db, MaybeAmbiguousPath::from_path(self))
    }
}

impl FugitiveSynNodePath {
    pub(super) fn new(
        db: &dyn EntitySynTreeDb,
        registry: &mut EntityNodeRegistry,
        path: FugitivePath,
    ) -> Self {
        Self::new_inner(db, registry.issue_maybe_ambiguous_path(path))
    }

    pub fn module_path(self, db: &dyn EntitySynTreeDb) -> ModulePath {
        self.maybe_ambiguous_path(db).path.module_path(db)
    }

    pub fn ident(self, db: &dyn EntitySynTreeDb) -> Ident {
        self.maybe_ambiguous_path(db).path.ident(db)
    }

    pub fn fugitive_kind(self, db: &dyn EntitySynTreeDb) -> FugitiveKind {
        self.maybe_ambiguous_path(db).path.fugitive_kind(db)
    }

    pub fn node(self, db: &dyn EntitySynTreeDb) -> ModuleItemSynNode {
        fugitive_syn_node(db, self)
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
pub(crate) fn fugitive_syn_node(
    db: &dyn EntitySynTreeDb,
    syn_node_path: FugitiveSynNodePath,
) -> ModuleItemSynNode {
    let module_path = syn_node_path.module_path(db);
    let item_sheet = module_path.item_tree_sheet(db).expect("valid file");
    match item_sheet
        .major_item_node(syn_node_path.into())
        .expect("should be some")
    {
        EntitySynNode::ModuleItem(node) => node,
        _ => unreachable!(),
    }
}
