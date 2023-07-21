use husky_entity_taxonomy::TypeKind;
use husky_print_utils::p;
use super::*;

#[salsa::interned(db = EntityTreeDb, jar = EntityTreeJar, constructor = new_inner)]
pub struct TypeSynNodePath {
    pub maybe_ambiguous_path: MaybeAmbiguousPath<TypePath>,
}

impl TypeSynNodePath {
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

    pub fn ty_kind(self,db:&dyn EntityTreeDb) -> TypeKind {
        self.maybe_ambiguous_path(db).path.ty_kind(db)
    }

    pub fn node<'a>(self, db: &'a dyn EntityTreeDb) -> ModuleItemSynNode {
        ty_node(db, self)
    }
}

impl HasSynNodePath for TypePath {
    type SynNodePath = TypeSynNodePath;

    fn node_path(self, db: &dyn EntityTreeDb) -> Self::SynNodePath {
        TypeSynNodePath::new_inner(db, MaybeAmbiguousPath::from_path(self))
    }
}

impl From<TypeSynNodePath> for EntitySynNodePath {
    fn from(id: TypeSynNodePath) -> Self {
        EntitySynNodePath::ModuleItem(id.into())
    }
}

#[salsa::tracked(jar = EntityTreeJar)]
pub(crate) fn ty_node(db: &dyn EntityTreeDb, node_path: TypeSynNodePath) -> ModuleItemSynNode {
    let module_path = node_path.module_path(db);
    // it's important to use presheet instead of sheet
    // otherwise cyclic when use all type variant paths
    let entity_presheet = db
        .entity_tree_presheet(module_path)
        .expect("should correspond to valid node");
    let Some(major_entity_node) = entity_presheet
        .major_entity_node(node_path.into()) else { 
        p!(node_path.debug(db));
        unreachable!("should be some, must be some erros in library")
    };
    match major_entity_node
    {
        EntitySynNode::ModuleItem(node) => node,
        _ => unreachable!(),
    }
}
