use super::*;

#[salsa::interned(db = EntityTreeDb, jar = EntityTreeJar, constructor = new_inner)]
pub struct TypeNodePath {
    pub maybe_ambiguous_path: MaybeAmbiguousPath<TypePath>,
}

impl TypeNodePath {
    pub(super) fn new(
        db: &dyn EntityTreeDb,
        registry: &mut EntityNodeRegistry,
        path: TypePath,
    ) -> Self {
        Self::new_inner(db, registry.issue_maybe_ambiguous_path(path))
    }

    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        self.maybe_ambiguous_path(db).path.module_path(db)
    }

    pub fn path(self, db: &dyn EntityTreeDb) -> Option<TypePath> {
        self.maybe_ambiguous_path(db).unambiguous_path()
    }

    pub fn node<'a>(self, db: &'a dyn EntityTreeDb) -> ModuleItemNode {
        ty_node(db, self)
    }
}

impl HasNodePath for TypePath {
    type NodePath = TypeNodePath;

    fn node_path(self, db: &dyn EntityTreeDb) -> Self::NodePath {
        TypeNodePath::new_inner(db, MaybeAmbiguousPath::from_path(self))
    }
}

impl From<TypeNodePath> for EntityNodePath {
    fn from(id: TypeNodePath) -> Self {
        EntityNodePath::ModuleItem(id.into())
    }
}

#[salsa::tracked(jar = EntityTreeJar)]
pub(crate) fn ty_node(db: &dyn EntityTreeDb, node_path: TypeNodePath) -> ModuleItemNode {
    let module_path = node_path.module_path(db);
    let entity_sheet = module_path
        .entity_tree_sheet(db)
        .expect("should correspond to valid node");

    match entity_sheet
        .major_entity_node(node_path.into())
        .expect("should be some")
    {
        EntityNode::ModuleItem(node) => node,
        _ => unreachable!(),
    }
}
