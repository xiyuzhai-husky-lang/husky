use super::*;

use vec_like::SmallVecPairMap;

#[salsa::interned(db = EntitySynTreeDb, jar = EntitySynTreeJar, constructor = new_inner)]
pub struct TraitSynNodePath {
    pub maybe_ambiguous_path: MaybeAmbiguousPath<TraitPath>,
}

impl From<TraitSynNodePath> for ItemSynNodePath {
    fn from(id: TraitSynNodePath) -> Self {
        ItemSynNodePath::MajorItem(id.into())
    }
}

// impl HasModulePath<Db> for TraitSynNodePath
// where
//     Db: ?Sized + EntitySynTreeDb,
// {
//     fn module_path(self, db: &Db) -> ModulePath {
//         let db = entity_syn_tree_db(db);
//         self.maybe_ambiguous_path(db).path.module_path(db)
//     }
// }

impl TraitSynNodePath {
    pub(super) fn new(
        db: &dyn EntitySynTreeDb,
        registry: &mut ItemSynNodePathRegistry,
        path: TraitPath,
    ) -> Self {
        Self::new_inner(db, registry.issue_maybe_ambiguous_path(path))
    }

    pub(crate) fn syn_node(self, db: &dyn EntitySynTreeDb) -> MajorItemSynNode {
        trai_node(db, self)
    }

    pub fn path(self, db: &dyn EntitySynTreeDb) -> TraitPath {
        self.maybe_ambiguous_path(db).path
    }

    // todo: make this a trait method
    pub(crate) fn associated_items<'a>(
        self,
        db: &'a dyn EntitySynTreeDb,
    ) -> &'a [(Ident, TraitItemSynNodePath, TraitItemSynNode)] {
        trai_item_syn_nodes(db, self)
    }

    pub fn item_node_paths<'a>(
        self,
        db: &'a dyn EntitySynTreeDb,
    ) -> impl Iterator<Item = TraitItemSynNodePath> + 'a {
        self.associated_items(db)
            .iter()
            .copied()
            .map(|(_, syn_node_path, _)| syn_node_path)
    }
}

impl HasSynNodePath for TraitPath {
    type SynNodePath = TraitSynNodePath;

    fn syn_node_path(self, db: &dyn EntitySynTreeDb) -> Self::SynNodePath {
        TraitSynNodePath::new_inner(db, MaybeAmbiguousPath::from_path(self))
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn trai_node(db: &dyn EntitySynTreeDb, syn_node_path: TraitSynNodePath) -> MajorItemSynNode {
    let module_path: ModulePath = todo!(); // syn_node_path.module_path(db);
    let item_sheet = module_path.item_tree_sheet(db);
    match item_sheet
        .major_item_node(syn_node_path.into())
        .expect("should be some")
    {
        ItemSynNode::MajorItem(node) => node,
        _ => unreachable!(),
    }
}

impl HasAssociatedItemPaths for TraitPath {
    type AssociatedItemPath = TraitItemPath;

    fn associated_item_paths(
        self,
        db: &dyn EntitySynTreeDb,
    ) -> &[(Ident, Self::AssociatedItemPath)] {
        trai_item_paths(db, self)
    }
}

#[salsa::tracked(jar = EntitySynTreeJar, return_ref)]
fn trai_item_paths(
    db: &dyn EntitySynTreeDb,
    path: TraitPath,
) -> SmallVecPairMap<Ident, TraitItemPath, APPROXIMATE_UPPER_BOUND_ON_NUMBER_OF_TRAIT_ITEMS> {
    let item_nodes = path.syn_node_path(db).associated_items(db);
    item_nodes
        .iter()
        .filter_map(|(ident, syn_node_path, _)| Some((*ident, syn_node_path.path(db)?)))
        .collect()
}
