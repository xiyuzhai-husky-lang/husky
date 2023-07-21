use super::*;
use smallvec::SmallVec;
use vec_like::{SmallVecMap, SmallVecPairMap};

#[salsa::interned(db = EntityTreeDb, jar = EntityTreeJar, constructor = new_inner)]
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
        db: &dyn EntityTreeDb,
        registry: &mut EntityNodeRegistry,
        path: TraitPath,
    ) -> Self {
        Self::new_inner(db, registry.issue_maybe_ambiguous_path(path))
    }

    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        self.maybe_ambiguous_path(db).path.module_path(db)
    }

    pub fn node(self, db: &dyn EntityTreeDb) -> ModuleItemSynNode {
        trai_node(db, self)
    }

    pub fn path(self, db: &dyn EntityTreeDb) -> TraitPath {
        self.maybe_ambiguous_path(db).path
    }

    // todo: make this a trait method
    pub fn item_nodes<'a>(
        self,
        db: &'a dyn EntityTreeDb,
    ) -> &'a [(Ident, TraitItemSynNodePath, TraitItemNode)] {
        trai_item_nodes(db, self)
    }
}

impl HasSynNodePath for TraitPath {
    type SynNodePath = TraitSynNodePath;

    fn syn_node_path(self, db: &dyn EntityTreeDb) -> Self::SynNodePath {
        TraitSynNodePath::new_inner(db, MaybeAmbiguousPath::from_path(self))
    }
}

#[salsa::tracked(jar = EntityTreeJar)]
fn trai_node(db: &dyn EntityTreeDb, node_path: TraitSynNodePath) -> ModuleItemSynNode {
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

impl HasItemPaths for TraitPath {
    type ItemPath = TraitItemPath;

    fn item_paths(self, db: &dyn EntityTreeDb) -> &[(Ident, Self::ItemPath)] {
        trai_item_paths(db, self)
    }
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
fn trai_item_paths(
    db: &dyn EntityTreeDb,
    path: TraitPath,
) -> SmallVecPairMap<Ident, TraitItemPath, APPROXIMATE_UPPER_BOUND_ON_NUMBER_OF_TRAIT_ITEMS> {
    let item_nodes = path.syn_node_path(db).item_nodes(db);
    item_nodes
        .iter()
        .filter_map(|(ident, node_path, _)| Some((*ident, node_path.path(db)?)))
        .collect()
}
