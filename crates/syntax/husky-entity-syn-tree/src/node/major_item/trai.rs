use super::*;

use vec_like::SmallVecPairMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::as_id(jar = EntitySynTreeJar)]
#[salsa::deref_id]
pub struct TraitSynNodePath(ItemSynNodePathId);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TraitSynNodePathData {
    pub maybe_ambiguous_path: MaybeAmbiguousPath<TraitPath>,
}

impl From<TraitSynNodePath> for ItemSynNodePath {
    fn from(id: TraitSynNodePath) -> Self {
        ItemSynNodePath::MajorItem(id.into())
    }
}

// impl HasModulePath<Db> for TraitSynNodePath
// where
//      + EntitySynTreeDb,
// {
//     fn module_path(self, db: &::salsa::Db,) -> ModulePath {
//         let db = entity_syn_tree_db(db);
//         self.maybe_ambiguous_path(db).path.module_path(db)
//     }
// }

impl TraitSynNodePath {
    pub(super) fn new(
        db: &::salsa::Db,
        registry: &mut ItemSynNodePathRegistry,
        path: TraitPath,
    ) -> Self {
        Self(ItemSynNodePathId::new(
            db,
            ItemSynNodePathData::MajorItem(MajorItemSynNodePathData::Trait(TraitSynNodePathData {
                maybe_ambiguous_path: registry.issue_maybe_ambiguous_path(path),
            })),
        ))
    }

    pub fn data(self, db: &::salsa::Db) -> TraitSynNodePathData {
        match self.0.data(db) {
            ItemSynNodePathData::MajorItem(MajorItemSynNodePathData::Trait(data)) => data,
            _ => unreachable!(),
        }
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
    pub(crate) fn associated_items<'a>(
        self,
        db: &'a ::salsa::Db,
    ) -> &'a [(Ident, TraitItemSynNodePath, TraitItemSynNode)] {
        trai_item_syn_nodes(db, self)
    }

    pub fn item_node_paths<'a>(
        self,
        db: &'a ::salsa::Db,
    ) -> impl Iterator<Item = TraitItemSynNodePath> + 'a {
        self.associated_items(db)
            .iter()
            .map(|&(_, syn_node_path, _)| syn_node_path)
    }
}

impl TraitSynNodePathData {
    pub fn path(self, db: &::salsa::Db) -> Option<TraitPath> {
        self.maybe_ambiguous_path.unambiguous_path()
    }
}

impl HasSynNodePath for TraitPath {
    type SynNodePath = TraitSynNodePath;

    fn syn_node_path(self, db: &::salsa::Db) -> Self::SynNodePath {
        TraitSynNodePath(ItemSynNodePathId::new(
            db,
            ItemSynNodePathData::MajorItem(MajorItemSynNodePathData::Trait(TraitSynNodePathData {
                maybe_ambiguous_path: MaybeAmbiguousPath::from_path(self),
            })),
        ))
    }
}

impl HasAssociatedItemPaths for TraitPath {
    type AssociatedItemPath = TraitItemPath;

    fn associated_item_paths(self, db: &::salsa::Db) -> &[(Ident, Self::AssociatedItemPath)] {
        trai_item_paths(db, self)
    }
}

#[salsa::tracked(jar = EntitySynTreeJar, return_ref)]
fn trai_item_paths(
    db: &::salsa::Db,
    path: TraitPath,
) -> SmallVecPairMap<Ident, TraitItemPath, APPROXIMATE_UPPER_BOUND_ON_NUMBER_OF_TRAIT_ITEMS> {
    let item_nodes = path.syn_node_path(db).associated_items(db);
    item_nodes
        .iter()
        .filter_map(|&(ident, syn_node_path, _)| Some((ident, syn_node_path.path(db)?)))
        .collect()
}
