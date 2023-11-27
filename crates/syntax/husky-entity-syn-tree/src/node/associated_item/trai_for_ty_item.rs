use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::as_id]
#[salsa::deref_id]
pub struct TraitForTypeItemSynNodePath(ItemSynNodePathId);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TraitForTypeItemSynNodePathData {
    maybe_ambiguous_path: MaybeAmbiguousPath<TraitForTypeItemPath>,
}

impl From<TraitForTypeItemSynNodePath> for ItemSynNodePath {
    fn from(path: TraitForTypeItemSynNodePath) -> Self {
        ItemSynNodePath::AssociatedItem(path.into())
    }
}

impl TraitForTypeItemSynNodePath {
    fn new(
        db: &::salsa::Db,
        registry: &mut ItemSynNodePathRegistry,
        path: TraitForTypeItemPath,
    ) -> Self {
        Self(ItemSynNodePathId::new(
            db,
            ItemSynNodePathData::AssociatedItem(AssociatedItemSynNodePathData::TraitForTypeItem(
                TraitForTypeItemSynNodePathData {
                    maybe_ambiguous_path: registry.issue_maybe_ambiguous_path(path),
                },
            )),
        ))
    }

    pub fn data(self, db: &::salsa::Db) -> TraitForTypeItemSynNodePathData {
        match self.0.data(db) {
            ItemSynNodePathData::AssociatedItem(
                AssociatedItemSynNodePathData::TraitForTypeItem(data),
            ) => data,
            _ => unreachable!(),
        }
    }

    pub fn path(self, db: &::salsa::Db) -> Option<TraitForTypeItemPath> {
        Some(match self.0.path(db)? {
            ItemPath::AssociatedItem(AssociatedItemPath::TraitForTypeItem(path)) => path,
            _ => unreachable!(),
        })
    }
}

impl TraitForTypeItemSynNodePathData {
    pub fn path(&self, db: &::salsa::Db) -> Option<TraitForTypeItemPath> {
        self.maybe_ambiguous_path.unambiguous_path()
    }

    pub fn impl_block(&self, db: &::salsa::Db) -> TraitForTypeImplBlockSynNodePath {
        self.maybe_ambiguous_path
            .path
            .impl_block(db)
            .syn_node_path(db)
    }

    pub fn item_kind(&self, db: &::salsa::Db) -> TraitItemKind {
        self.maybe_ambiguous_path.path.item_kind(db)
    }

    pub(crate) fn syn_node<'a>(
        &'a self,
        syn_node_path: TraitForTypeItemSynNodePath,
        db: &'a ::salsa::Db,
    ) -> &'a TraitForTypeItemSynNode {
        self.impl_block(db)
            .associated_items(db)
            .iter()
            .find_map(|&(_, node_path1, ref node)| (node_path1 == syn_node_path).then_some(node))
            .expect("some")
    }
}

impl HasSynNodePath for TraitForTypeItemPath {
    type SynNodePath = TraitForTypeItemSynNodePath;

    fn syn_node_path(self, db: &::salsa::Db) -> Self::SynNodePath {
        TraitForTypeItemSynNodePath(ItemSynNodePathId::new(
            db,
            ItemSynNodePathData::AssociatedItem(AssociatedItemSynNodePathData::TraitForTypeItem(
                TraitForTypeItemSynNodePathData {
                    maybe_ambiguous_path: MaybeAmbiguousPath::from_path(self),
                },
            )),
        ))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct TraitForTypeItemSynNode {
    syn_node_path: TraitForTypeItemSynNodePath,
    ast_idx: AstIdx,
    ident: Ident,
    item_kind: TraitItemKind,
    visibility: Scope,
    is_generic: bool,
}

impl TraitForTypeItemSynNode {
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
        let path =
            TraitForTypeItemPath::new(impl_block_syn_node_path.path(db), ident, item_kind, db);
        let syn_node_path = TraitForTypeItemSynNodePath::new(db, registry, path);
        (
            syn_node_path,
            Self {
                syn_node_path,
                ast_idx,
                ident,
                item_kind,
                visibility,
                is_generic,
            },
        )
    }
}
