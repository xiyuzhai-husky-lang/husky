use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TraitForTypeItemSynNodePath(ItemSynNodePathId);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TraitForTypeItemSynNodePathData {
    maybe_ambiguous_path: MaybeAmbiguousPath<TraitForTypeItemPath>,
}

impl TraitForTypeItemSynNodePath {
    fn new(
        db: &::salsa::Db,
        registry: &mut ItemSynNodePathRegistry,
        path: TraitForTypeItemPath,
    ) -> Self {
        Self::new_inner(db, registry.issue_maybe_ambiguous_path(path))
    }

    pub fn path(self, db: &::salsa::Db) -> Option<TraitForTypeItemPath> {
        self.maybe_ambiguous_path(db).unambiguous_path()
    }

    pub fn impl_block(self, db: &::salsa::Db) -> TraitForTypeImplBlockSynNodePath {
        self.maybe_ambiguous_path(db)
            .path
            .impl_block(db)
            .syn_node_path(db)
    }

    pub fn item_kind(self, db: &::salsa::Db) -> TraitItemKind {
        self.maybe_ambiguous_path(db).path.item_kind(db)
    }

    pub(crate) fn syn_node(self, db: &::salsa::Db) -> TraitForTypeItemSynNodeData {
        trai_for_ty_item_syn_node(db, self)
    }
}

// impl HasModulePath<Db> for TraitForTypeItemSynNodePath
// where
//      + EntitySynTreeDb,
// {
//     fn module_path(self, db: &::salsa::Db,) -> ModulePath {
//         let db = entity_syn_tree_db(db);
//         self.maybe_ambiguous_path(db).path.module_path(db)
//     }
// }

impl From<TraitForTypeItemSynNodePath> for ItemSynNodePath {
    fn from(id: TraitForTypeItemSynNodePath) -> Self {
        ItemSynNodePath::AssociatedItem(id.into())
    }
}

impl HasSynNodePath for TraitForTypeItemPath {
    type SynNodePath = TraitForTypeItemSynNodePath;

    fn syn_node_path(self, db: &::salsa::Db) -> Self::SynNodePath {
        TraitForTypeItemSynNodePath::new_inner(db, MaybeAmbiguousPath::from_path(self))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct TraitForTypeItemSynNodeData {
    syn_node_path: TraitForTypeItemSynNodePath,
    ast_idx: AstIdx,
    ident: Ident,
    item_kind: TraitItemKind,
    visibility: Scope,
    is_generic: bool,
}

impl TraitForTypeItemSynNodeData {
    #[inline(always)]
    pub(crate) fn new(
        db: &::salsa::Db,
        registry: &mut ItemSynNodePathRegistry,
        impl_block_syn_node_path: TraitForTypeImplBlockSynNodePath,
        ast_idx: AstIdx,
        ident: Ident,
        item_kind: TraitItemKind,
        visibility: Scope,
        is_generic: bool,
    ) -> (TraitForTypeItemSynNodePath, Self) {
        let path = TraitForTypeItemPath::new(impl_block_syn_node_path.path(), ident, item_kind, db);
        let syn_node_path = TraitForTypeItemSynNodePath::new(db, registry, path);
        (
            syn_node_path,
            Self::new_inner(
                db,
                syn_node_path,
                ast_idx,
                ident,
                item_kind,
                visibility,
                is_generic,
            ),
        )
    }
}

pub(crate) fn trai_for_ty_item_syn_node(
    db: &::salsa::Db,
    syn_node_path: TraitForTypeItemSynNodePath,
) -> TraitForTypeItemSynNodeData {
    syn_node_path
        .impl_block(db)
        .associated_items(db)
        .iter()
        .copied()
        .find_map(|(_, node_path1, node)| (node_path1 == syn_node_path).then_some(node))
        .expect("some")
}
