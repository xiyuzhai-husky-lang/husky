use super::*;
use smallvec::SmallVec;
use vec_like::{SmallVecMap, SmallVecPairMap};

#[salsa::interned(db = EntitySynTreeDb, jar = EntitySynTreeJar, constructor = new_inner)]
pub struct TraitSynNodePath {
    pub maybe_ambiguous_path: MaybeAmbiguousPath<TraitPath>,
}

impl From<TraitSynNodePath> for EntitySynNodePath {
    fn from(id: TraitSynNodePath) -> Self {
        EntitySynNodePath::ModuleItem(id.into())
    }
}

impl TraitSynNodePath {
    pub(super) fn new(
        db: &dyn EntitySynTreeDb,
        registry: &mut EntityNodeRegistry,
        path: TraitPath,
    ) -> Self {
        Self::new_inner(db, registry.issue_maybe_ambiguous_path(path))
    }

    pub fn module_path(self, db: &dyn EntitySynTreeDb) -> ModulePath {
        self.maybe_ambiguous_path(db).path.module_path(db)
    }

    pub fn node(self, db: &dyn EntitySynTreeDb) -> ModuleItemSynNode {
        trai_node(db, self)
    }

    pub fn path(self, db: &dyn EntitySynTreeDb) -> TraitPath {
        self.maybe_ambiguous_path(db).path
    }

    // todo: make this a trait method
    pub fn item_nodes<'a>(
        self,
        db: &'a dyn EntitySynTreeDb,
    ) -> &'a [(Ident, TraitItemSynNodePath, TraitItemSynNode)] {
        trai_item_syn_nodes(db, self)
    }
}

impl HasSynNodePath for TraitPath {
    type SynNodePath = TraitSynNodePath;

    fn syn_node_path(self, db: &dyn EntitySynTreeDb) -> Self::SynNodePath {
        TraitSynNodePath::new_inner(db, MaybeAmbiguousPath::from_path(self))
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn trai_node(db: &dyn EntitySynTreeDb, syn_node_path: TraitSynNodePath) -> ModuleItemSynNode {
    let module_path = syn_node_path.module_path(db);
    let entity_sheet = module_path.entity_tree_sheet(db).expect("valid file");
    match entity_sheet
        .major_entity_node(syn_node_path.into())
        .expect("should be some")
    {
        EntitySynNode::ModuleItem(node) => node,
        _ => unreachable!(),
    }
}

impl HasItemPaths for TraitPath {
    type ItemPath = TraitItemPath;

    fn item_paths(self, db: &dyn EntitySynTreeDb) -> &[(Ident, Self::ItemPath)] {
        trai_item_paths(db, self)
    }
}

#[salsa::tracked(jar = EntitySynTreeJar, return_ref)]
fn trai_item_paths(
    db: &dyn EntitySynTreeDb,
    path: TraitPath,
) -> SmallVecPairMap<Ident, TraitItemPath, APPROXIMATE_UPPER_BOUND_ON_NUMBER_OF_TRAIT_ITEMS> {
    let item_nodes = path.syn_node_path(db).item_nodes(db);
    item_nodes
        .iter()
        .filter_map(|(ident, syn_node_path, _)| Some((*ident, syn_node_path.path(db)?)))
        .collect()
}
