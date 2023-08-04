use super::*;
use husky_entity_taxonomy::TypeKind;
use husky_print_utils::p;

#[salsa::interned(db = EntitySynTreeDb, jar = EntitySynTreeJar, constructor = new_inner)]
pub struct TypeSynNodePath {
    pub maybe_ambiguous_path: MaybeAmbiguousPath<TypePath>,
}

impl TypeSynNodePath {
    pub(super) fn new(
        db: &dyn EntitySynTreeDb,
        registry: &mut ItemNodeRegistry,
        path: TypePath,
    ) -> Self {
        Self::new_inner(db, registry.issue_maybe_ambiguous_path(path))
    }

    pub fn module_path(self, db: &dyn EntitySynTreeDb) -> ModulePath {
        self.maybe_ambiguous_path(db).path.module_path(db)
    }

    pub fn path(self, db: &dyn EntitySynTreeDb) -> Option<TypePath> {
        self.maybe_ambiguous_path(db).unambiguous_path()
    }

    pub fn ty_kind(self, db: &dyn EntitySynTreeDb) -> TypeKind {
        self.maybe_ambiguous_path(db).path.ty_kind(db)
    }

    pub fn node<'a>(self, db: &'a dyn EntitySynTreeDb) -> MajorItemSynNode {
        ty_node(db, self)
    }
}

impl HasSynNodePath for TypePath {
    type SynNodePath = TypeSynNodePath;

    fn syn_node_path(self, db: &dyn EntitySynTreeDb) -> Self::SynNodePath {
        TypeSynNodePath::new_inner(db, MaybeAmbiguousPath::from_path(self))
    }
}

impl From<TypeSynNodePath> for ItemSynNodePath {
    fn from(id: TypeSynNodePath) -> Self {
        ItemSynNodePath::MajorItem(id.into())
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
pub(crate) fn ty_node(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TypeSynNodePath,
) -> MajorItemSynNode {
    let module_path = syn_node_path.module_path(db);
    // it's important to use presheet instead of sheet
    // otherwise cyclic when use all type variant paths
    let item_presheet = db
        .item_syn_tree_presheet(module_path)
        .expect("should correspond to valid node");
    let Some(major_item_node) = item_presheet.major_item_node(syn_node_path.into()) else {
        p!(syn_node_path.debug(db));
        unreachable!("should be some, must be some erros in library")
    };
    match major_item_node {
        ItemSynNode::MajorItem(node) => node,
        _ => unreachable!(),
    }
}
