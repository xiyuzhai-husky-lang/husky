use super::*;

#[salsa::interned(db = EntityTreeDb, jar = EntityTreeJar, constructor = new_inner)]
pub struct FugitiveNodeId {
    pub maybe_ambiguous_path: MaybeAmbiguousPath<FugitivePath>,
}

impl From<FugitiveNodeId> for EntityNodeId {
    fn from(id: FugitiveNodeId) -> Self {
        EntityNodeId::ModuleItem(id.into())
    }
}

impl HasNodeId for FugitivePath {
    type NodeId = FugitiveNodeId;

    fn node_id(self, db: &dyn EntityTreeDb) -> Self::NodeId {
        FugitiveNodeId::new_inner(db, MaybeAmbiguousPath::from_path(self))
    }
}

impl FugitiveNodeId {
    pub(super) fn new(
        db: &dyn EntityTreeDb,
        registry: &mut EntityNodeRegistry,
        path: FugitivePath,
    ) -> Self {
        Self::new_inner(db, registry.issue_maybe_ambiguous(path))
    }

    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        self.maybe_ambiguous_path(db).path.module_path(db)
    }

    pub fn node(self, db: &dyn EntityTreeDb) -> ModuleItemNode {
        fugitive_node(db, self)
    }
}

#[salsa::tracked(jar = EntityTreeJar)]
pub(crate) fn fugitive_node(db: &dyn EntityTreeDb, node_id: FugitiveNodeId) -> ModuleItemNode {
    let module_path = node_id.module_path(db);
    let entity_sheet = module_path.entity_tree_sheet(db).expect("valid file");
    match entity_sheet
        .major_entity_node(node_id.into())
        .expect("should be some")
    {
        EntityNode::ModuleItem(node) => node,
        _ => unreachable!(),
    }
}
