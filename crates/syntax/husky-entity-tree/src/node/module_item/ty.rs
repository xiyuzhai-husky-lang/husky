use super::*;

#[salsa::interned(db = EntityTreeDb, jar = EntityTreeJar, constructor = new_inner)]
pub struct TypeNodeId {
    pub maybe_ambiguous_path: MaybeAmbiguousPath<TypePath>,
}

impl TypeNodeId {
    pub(super) fn new(
        db: &dyn EntityTreeDb,
        registry: &mut EntityNodeRegistry,
        path: TypePath,
    ) -> Self {
        Self::new_inner(db, registry.issue_maybe_ambiguous(path))
    }

    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        self.maybe_ambiguous_path(db).path.module_path(db)
    }

    pub fn path(self, db: &dyn EntityTreeDb) -> Option<TypePath> {
        self.maybe_ambiguous_path(db).unambiguous_path()
    }

    pub fn node<'a>(self, db: &'a dyn EntityTreeDb) -> EntityTreeResultRef<'a, ModuleItemNode> {
        ty_node(db, self).as_ref().copied()
    }
}

impl HasNodeId for TypePath {
    type NodeId = TypeNodeId;

    fn node_id(self, db: &dyn EntityTreeDb) -> Self::NodeId {
        TypeNodeId::new_inner(db, MaybeAmbiguousPath::from_path(self))
    }
}

impl From<TypeNodeId> for EntityNodeId {
    fn from(id: TypeNodeId) -> Self {
        EntityNodeId::ModuleItem(id.into())
    }
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn ty_node(
    db: &dyn EntityTreeDb,
    node_id: TypeNodeId,
) -> EntityTreeResult<ModuleItemNode> {
    let module_path = node_id.module_path(db);
    let entity_sheet = module_path.entity_tree_sheet(db)?;
    Ok(
        match entity_sheet
            .major_entity_node(node_id.into())
            .expect("should be some")
        {
            EntityNode::ModuleItem(node) => node,
            _ => unreachable!(),
        },
    )
}
