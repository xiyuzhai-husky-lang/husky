use husky_entity_kind::FugitiveKind;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FugitiveSynNodePath(ItemSynNodePathId);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FugitiveSynNodePathData {
    pub maybe_ambiguous_path: MaybeAmbiguousPath<FugitivePath>,
}

impl From<FugitiveSynNodePath> for ItemSynNodePath {
    fn from(id: FugitiveSynNodePath) -> Self {
        ItemSynNodePath::MajorItem(id.into())
    }
}

impl HasSynNodePath for FugitivePath {
    type SynNodePath = FugitiveSynNodePath;

    fn syn_node_path(self, db: &::salsa::Db) -> Self::SynNodePath {
        FugitiveSynNodePath::new_inner(db, MaybeAmbiguousPath::from_path(self))
    }
}

impl FugitiveSynNodePath {
    pub(super) fn new(
        db: &::salsa::Db,
        registry: &mut ItemSynNodePathRegistry,
        path: FugitivePath,
    ) -> Self {
        Self::new_inner(db, registry.issue_maybe_ambiguous_path(path))
    }

    pub fn ident(self, db: &::salsa::Db) -> Ident {
        self.maybe_ambiguous_path(db).path.ident(db)
    }

    pub fn fugitive_kind(self, db: &::salsa::Db) -> FugitiveKind {
        self.maybe_ambiguous_path(db).path.fugitive_kind(db)
    }

    pub(crate) fn syn_node(self, db: &::salsa::Db) -> MajorItemSynNodeData {
        fugitive_syn_node(db, self)
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
pub(crate) fn fugitive_syn_node(
    db: &::salsa::Db,
    syn_node_path: FugitiveSynNodePath,
) -> MajorItemSynNodeData {
    let module_path: ModulePath = todo!(); //syn_node_path.module_path(db);
    let item_sheet = module_path.item_tree_sheet(db);
    match item_sheet
        .major_item_node(syn_node_path.into())
        .expect("should be some")
    {
        ItemSynNode::MajorItem(node) => node,
        _ => unreachable!(),
    }
}
