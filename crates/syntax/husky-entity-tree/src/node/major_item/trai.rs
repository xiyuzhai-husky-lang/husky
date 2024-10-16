use super::*;
use crate::node::{
    assoc_item::trai_item::APPROXIMATE_UPPER_BOUND_ON_NUMBER_OF_TRAIT_ITEMS,
    trai_item::{trai_item_syn_nodes, TraitItemSynNode, TraitItemSynNodePath},
};
use husky_entity_path::path::{assoc_item::trai_item::TraitItemPath, major_item::trai::TraitPath};
use vec_like::SmallVecPairMap;

#[salsa::derive_debug_with_db]
#[salsa::as_id(jar = EntityTreeJar)]
#[salsa::deref_id]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TraitSynNodePath(ItemSynNodePathId);

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TraitSynNodePathData {
    pub disambiguated_item_path: DisambiguatedItemPath<TraitPath>,
}

impl From<TraitSynNodePath> for ItemSynNodePath {
    fn from(id: TraitSynNodePath) -> Self {
        ItemSynNodePath::MajorItem(id.into())
    }
}

impl TraitSynNodePath {
    pub(super) fn new(
        db: &::salsa::Db,
        registry: &mut ItemSynNodePathRegistry,
        path: TraitPath,
    ) -> Self {
        Self(ItemSynNodePathId::new(
            db,
            ItemSynNodePathData::MajorItem(MajorItemSynNodePathData::Trait(TraitSynNodePathData {
                disambiguated_item_path: registry.issue_maybe_ambiguous_path(path),
            })),
        ))
    }

    pub fn data(self, db: &::salsa::Db) -> TraitSynNodePathData {
        match self.0.data(db) {
            ItemSynNodePathData::MajorItem(MajorItemSynNodePathData::Trait(data)) => data,
            _ => unreachable!(),
        }
    }

    pub fn unambiguous_item_path(self, db: &::salsa::Db) -> Option<TraitPath> {
        self.data(db)
            .disambiguated_item_path
            .unambiguous_item_path()
    }

    pub fn ident(self, db: &::salsa::Db) -> Ident {
        self.data(db)
            .disambiguated_item_path
            .maybe_ambiguous_item_path
            .ident(db)
    }

    pub(crate) fn syn_node<'a>(self, db: &'a ::salsa::Db) -> &'a MajorItemSynNode {
        let module_path = self.module_path(db);
        let item_sheet = module_path.item_tree_sheet(db);
        match item_sheet
            .major_item_node(self.into())
            .expect("should be some")
        {
            ItemSynNode::MajorItem(node) => node,
            _ => unreachable!(),
        }
    }

    // todo: make this a trait method
    pub(crate) fn assoc_items<'a>(
        self,
        db: &'a ::salsa::Db,
    ) -> &'a [(Ident, TraitItemSynNodePath, TraitItemSynNode)] {
        trai_item_syn_nodes(db, self)
    }

    pub fn item_node_paths<'a>(
        self,
        db: &'a ::salsa::Db,
    ) -> impl Iterator<Item = TraitItemSynNodePath> + 'a {
        self.assoc_items(db)
            .iter()
            .map(|&(_, syn_node_path, _)| syn_node_path)
    }
}

impl TraitSynNodePathData {
    pub fn syn_node_path(self, id: ItemSynNodePathId) -> TraitSynNodePath {
        TraitSynNodePath(id)
    }

    pub fn path(self) -> Option<TraitPath> {
        self.disambiguated_item_path.unambiguous_item_path()
    }

    pub fn module_path(self, db: &::salsa::Db) -> ModulePath {
        self.disambiguated_item_path
            .maybe_ambiguous_item_path
            .module_path(db)
    }

    pub fn ast_idx(self, id: ItemSynNodePathId, db: &::salsa::Db) -> AstIdx {
        TraitSynNodePath(id).syn_node(db).ast_idx
    }
}

impl HasSynNodePath for TraitPath {
    type SynNodePath = TraitSynNodePath;

    fn syn_node_path(self, db: &::salsa::Db) -> Self::SynNodePath {
        TraitSynNodePath(ItemSynNodePathId::new(
            db,
            ItemSynNodePathData::MajorItem(MajorItemSynNodePathData::Trait(TraitSynNodePathData {
                disambiguated_item_path: DisambiguatedItemPath::from_path(self),
            })),
        ))
    }
}

impl HasAssocItemPaths for TraitPath {
    type AssocItemPath = TraitItemPath;

    fn assoc_item_paths(self, db: &::salsa::Db) -> &[(Ident, Self::AssocItemPath)] {
        trai_item_paths(db, self)
    }
}

#[salsa::tracked(return_ref)]
fn trai_item_paths(
    db: &::salsa::Db,
    path: TraitPath,
) -> SmallVecPairMap<Ident, TraitItemPath, APPROXIMATE_UPPER_BOUND_ON_NUMBER_OF_TRAIT_ITEMS> {
    let item_nodes = path.syn_node_path(db).assoc_items(db);
    item_nodes
        .iter()
        .filter_map(|&(ident, syn_node_path, _)| Some((ident, syn_node_path.path(db)?)))
        .collect()
}
