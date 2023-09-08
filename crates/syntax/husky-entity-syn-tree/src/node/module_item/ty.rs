use super::*;
use husky_entity_taxonomy::TypeKind;
use husky_print_utils::p;
use smallvec::SmallVec;

#[salsa::interned(db = EntitySynTreeDb, jar = EntitySynTreeJar, constructor = new_inner)]
pub struct TypeSynNodePath {
    pub maybe_ambiguous_path: MaybeAmbiguousPath<TypePath>,
}

impl TypeSynNodePath {
    pub(super) fn new(
        db: &dyn EntitySynTreeDb,
        registry: &mut ItemSynNodePathRegistry,
        path: TypePath,
    ) -> Self {
        Self::new_inner(db, registry.issue_maybe_ambiguous_path(path))
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

    pub fn decrs<'a>(self, db: &'a dyn EntitySynTreeDb) -> &'a [(DecrSynNodePath, DecrSynNode)] {
        ty_decrs(db, self)
    }
}

impl<Db> HasModulePath<Db> for TypeSynNodePath
where
    Db: ?Sized + EntitySynTreeDb,
{
    fn module_path(self, db: &Db) -> ModulePath {
        let db = entity_syn_tree_db(db);
        self.maybe_ambiguous_path(db).path.module_path(db)
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
fn ty_node(db: &dyn EntitySynTreeDb, syn_node_path: TypeSynNodePath) -> MajorItemSynNode {
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

impl HasDecrPaths for TypePath {
    type DecrPath = DecrPath;

    fn decr_paths(self, db: &dyn EntitySynTreeDb) -> &[Self::DecrPath] {
        ty_decr_paths(db, self)
    }
}

#[salsa::tracked(jar = EntitySynTreeJar, return_ref)]
fn ty_decrs(
    db: &dyn EntitySynTreeDb,
    ty_syn_node_path: TypeSynNodePath,
) -> SmallVec<[(DecrSynNodePath, DecrSynNode); 2]> {
    let ast_sheet = ty_syn_node_path
        .module_path(db)
        .ast_sheet(db)
        .expect("todo: module paths should be guaranteed to be valid");
    let mut registry = ItemSynNodePathRegistry::default();
    ast_sheet.procure_decrs(
        ty_syn_node_path.maybe_ambiguous_path(db).path.into(),
        ty_syn_node_path.node(db).ast_idx(db),
        move |decr_ast_idx, _, path| {
            DecrSynNode::new(
                ty_syn_node_path.into(),
                path,
                decr_ast_idx,
                &mut registry,
                db,
            )
        },
        db,
    )
}

#[salsa::tracked(jar = EntitySynTreeJar, return_ref)]
fn ty_decr_paths(db: &dyn EntitySynTreeDb, path: TypePath) -> SmallVec<[DecrPath; 2]> {
    path.syn_node_path(db)
        .decrs(db)
        .iter()
        .filter_map(|(decr_syn_node_path, _)| decr_syn_node_path.path(db))
        .collect()
}
