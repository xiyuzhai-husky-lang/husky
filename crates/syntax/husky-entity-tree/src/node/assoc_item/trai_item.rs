use super::*;
use crate::node::trai::TraitSynNodePath;
use husky_entity_path::path::assoc_item::trai_item::TraitItemPath;
use smallvec::SmallVec;

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::as_id]
#[salsa::deref_id]
pub struct TraitItemSynNodePath(ItemSynNodePathId);

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TraitItemSynNodePathData {
    pub parent_trai_syn_node_path: TraitSynNodePath,
    disambiguated_item_path: DisambiguatedItemPath<TraitItemPath>,
}

impl From<TraitItemSynNodePath> for ItemSynNodePath {
    fn from(path: TraitItemSynNodePath) -> Self {
        ItemSynNodePath::AssocItem(path.into())
    }
}

impl TraitItemSynNodePath {
    fn new(
        db: &::salsa::Db,
        registry: &mut ItemSynNodePathRegistry,
        parent_trai_syn_node_path: TraitSynNodePath,
        path: TraitItemPath,
    ) -> Self {
        Self(ItemSynNodePathId::new(
            db,
            ItemSynNodePathData::AssocItem(AssocItemSynNodePathData::TraitItem(
                TraitItemSynNodePathData {
                    parent_trai_syn_node_path,
                    disambiguated_item_path: registry.issue_maybe_ambiguous_path(path),
                },
            )),
        ))
    }

    pub fn path(self, db: &::salsa::Db) -> Option<TraitItemPath> {
        Some(match self.0.unambiguous_item_path(db)? {
            ItemPath::AssocItem(AssocItemPath::TraitItem(path)) => path,
            _ => unreachable!(),
        })
    }

    pub fn data(self, db: &::salsa::Db) -> TraitItemSynNodePathData {
        match self.0.data(db) {
            ItemSynNodePathData::AssocItem(AssocItemSynNodePathData::TraitItem(data)) => data,
            _ => unreachable!(),
        }
    }

    pub fn parent_trai_syn_node_path(self, db: &::salsa::Db) -> TraitSynNodePath {
        self.data(db).parent_trai_syn_node_path
    }

    pub fn item_kind(self, db: &::salsa::Db) -> TraitItemKind {
        self.data(db).item_kind(db)
    }
}

impl TraitItemSynNodePathData {
    #[inline(always)]
    pub fn syn_node_path(self, id: ItemSynNodePathId) -> TraitItemSynNodePath {
        TraitItemSynNodePath(id)
    }

    pub fn path(self) -> Option<TraitItemPath> {
        self.disambiguated_item_path.unambiguous_item_path()
    }

    pub fn module_path(self, db: &::salsa::Db) -> ModulePath {
        self.disambiguated_item_path
            .maybe_ambiguous_item_path
            .module_path(db)
    }

    pub fn item_kind(self, db: &::salsa::Db) -> TraitItemKind {
        self.disambiguated_item_path
            .maybe_ambiguous_item_path
            .item_kind(db)
    }

    pub fn ast_idx(self, id: ItemSynNodePathId, db: &::salsa::Db) -> AstIdx {
        self.syn_node(TraitItemSynNodePath(id), db).ast_idx
    }

    pub(crate) fn syn_node<'a>(
        self,
        syn_node_path: TraitItemSynNodePath,
        db: &'a ::salsa::Db,
    ) -> &'a TraitItemSynNode {
        self.parent_trai_syn_node_path
            .assoc_items(db)
            .iter()
            .find_map(|&(_, node_path1, ref node)| (node_path1 == syn_node_path).then_some(node))
            .expect("some")
    }
}

impl HasSynNodePath for TraitItemPath {
    type SynNodePath = TraitItemSynNodePath;

    fn syn_node_path(self, db: &::salsa::Db) -> Self::SynNodePath {
        TraitItemSynNodePath(ItemSynNodePathId::new(
            db,
            ItemSynNodePathData::AssocItem(AssocItemSynNodePathData::TraitItem(
                TraitItemSynNodePathData {
                    parent_trai_syn_node_path: self.trai_path(db).syn_node_path(db),
                    disambiguated_item_path: DisambiguatedItemPath::from_path(self),
                },
            )),
        ))
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct TraitItemSynNode {
    syn_node_path: TraitItemSynNodePath,
    ast_idx: AstIdx,
    ident: Ident,
    item_kind: TraitItemKind,
    visibility: Scope,
    is_generic: bool,
}

/// # constructor
impl TraitItemSynNode {
    #[inline(always)]
    fn new(
        db: &::salsa::Db,
        registry: &mut ItemSynNodePathRegistry,
        trai_syn_node_path: TraitSynNodePath,
        ast_idx: AstIdx,
        ident: Ident,
        item_kind: TraitItemKind,
        visibility: Scope,
        is_generic: bool,
    ) -> (TraitItemSynNodePath, Self) {
        let trai_item_path = TraitItemPath::new(
            trai_syn_node_path
                .data(db)
                .disambiguated_item_path
                .maybe_ambiguous_item_path,
            ident,
            item_kind,
            db,
        );
        let syn_node_path =
            TraitItemSynNodePath::new(db, registry, trai_syn_node_path, trai_item_path);
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

#[salsa::tracked(return_ref)]
pub(crate) fn trai_item_syn_nodes(
    db: &::salsa::Db,
    trai_node_path: TraitSynNodePath,
) -> SmallVec<
    [(Ident, TraitItemSynNodePath, TraitItemSynNode);
        APPROXIMATE_UPPER_BOUND_ON_NUMBER_OF_TRAIT_ITEMS],
> {
    let trai_node = trai_node_path.syn_node(db);
    let module_path = trai_node_path.module_path(db);
    let ast_sheet = module_path.ast_sheet(db);
    let DefnBlock::Trait { path: _, items } = trai_node.block else {
        unreachable!()
    };
    let Some(items) = items else {
        return Default::default();
    };
    let mut registry = ItemSynNodePathRegistry::default();
    items
        .ast_idx_range()
        .into_iter()
        .filter_map(|ast_idx| {
            let ast = &ast_sheet[ast_idx];
            match ast {
                AstData::Identifiable {
                    visibility_expr,
                    item_kind,
                    ident_token,
                    is_generic,
                    ..
                } => {
                    let EntityKind::AssocItem {
                        assoc_item_kind: AssocItemKind::TraitItem(item_kind),
                    } = item_kind
                    else {
                        unreachable!()
                    };
                    let (syn_node_path, node) = TraitItemSynNode::new(
                        db,
                        &mut registry,
                        trai_node_path,
                        ast_idx,
                        ident_token.ident(),
                        *item_kind,
                        visibility_expr.visibility(),
                        *is_generic,
                    );
                    Some((ident_token.ident(), syn_node_path, node))
                }
                AstData::Err { .. } => None,
                _ => unreachable!(),
            }
        })
        .collect()
}

pub(crate) const APPROXIMATE_UPPER_BOUND_ON_NUMBER_OF_TRAIT_ITEMS: usize = 4;
