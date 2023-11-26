use super::*;
use husky_entity_kind::TypeKind;
use husky_print_utils::p;
use smallvec::SmallVec;

#[salsa::interned(db = EntitySynTreeDb, jar = EntitySynTreeJar, constructor = new_inner)]
pub struct TypeSynNodePath {
    pub maybe_ambiguous_path: MaybeAmbiguousPath<TypePath>,
}

impl TypeSynNodePath {
    pub(super) fn new(
        db: &::salsa::Db,
        registry: &mut ItemSynNodePathRegistry,
        path: TypePath,
    ) -> Self {
        Self::new_inner(db, registry.issue_maybe_ambiguous_path(path))
    }

    pub fn path(self, db: &::salsa::Db) -> Option<TypePath> {
        self.maybe_ambiguous_path(db).unambiguous_path()
    }

    pub fn ty_kind(self, db: &::salsa::Db) -> TypeKind {
        self.maybe_ambiguous_path(db).path.ty_kind(db)
    }

    pub(crate) fn syn_node<'a>(self, db: &'a ::salsa::Db) -> MajorItemSynNodeData {
        ty_node(db, self)
    }

    pub(crate) fn attr_syn_nodes<'a>(
        self,
        db: &'a ::salsa::Db,
    ) -> &'a [(AttrSynNodePath, AttrSynNodeData)] {
        ty_attrs(db, self)
    }
}

// impl  TypeSynNodePath
// where
//      + EntitySynTreeDb,
// {
//     fn module_path(self, db: &::salsa::Db,) -> ModulePath {
//         let db = entity_syn_tree_db(db);
//         self.maybe_ambiguous_path(db).path.module_path(db)
//     }
// }

impl HasSynNodePath for TypePath {
    type SynNodePath = TypeSynNodePath;

    fn syn_node_path(self, db: &::salsa::Db) -> Self::SynNodePath {
        TypeSynNodePath::new_inner(db, MaybeAmbiguousPath::from_path(self))
    }
}

impl From<TypeSynNodePath> for ItemSynNodePath {
    fn from(id: TypeSynNodePath) -> Self {
        ItemSynNodePath::MajorItem(id.into())
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
fn ty_node(db: &::salsa::Db, syn_node_path: TypeSynNodePath) -> MajorItemSynNodeData {
    let module_path: ModulePath = todo!(); //= syn_node_path.module_path(db);
                                           // it's important to use presheet instead of sheet
                                           // otherwise cyclic when use all type variant paths
    let item_presheet = db.item_syn_tree_presheet(module_path);
    let Some(major_item_node) = item_presheet.major_item_node(syn_node_path.into()) else {
        p!(syn_node_path.debug(db));
        unreachable!("should be some, must be some erros in library")
    };
    match major_item_node {
        ItemSynNode::MajorItem(node) => node,
        _ => unreachable!(),
    }
}

impl HasAttrPaths for TypePath {
    type AttrPath = AttrItemPath;

    fn attr_paths(self, db: &::salsa::Db) -> &[Self::AttrPath] {
        ty_attr_paths(db, self)
    }
}

#[salsa::tracked(jar = EntitySynTreeJar, return_ref)]
fn ty_attrs(
    db: &::salsa::Db,
    ty_syn_node_path: TypeSynNodePath,
) -> SmallVec<[(AttrSynNodePath, AttrSynNodeData); 2]> {
    let ast_sheet: AstSheet = todo!(); // = ty_syn_node_path.module_path(db).ast_sheet(db);
    let mut registry = ItemSynNodePathRegistry::default();
    ast_sheet.procure_attrs(
        ty_syn_node_path.maybe_ambiguous_path(db).path.into(),
        ty_syn_node_path.syn_node(db).ast_idx(db),
        move |attr_ast_idx, _, path| {
            AttrSynNodeData::new(
                ty_syn_node_path.into(),
                path,
                attr_ast_idx,
                &mut registry,
                db,
            )
        },
        db,
    )
}

#[salsa::tracked(jar = EntitySynTreeJar, return_ref)]
fn ty_attr_paths(db: &::salsa::Db, path: TypePath) -> SmallVec<[AttrItemPath; 2]> {
    path.syn_node_path(db)
        .attr_syn_nodes(db)
        .iter()
        .filter_map(|(attr_syn_node_path, _)| attr_syn_node_path.path(db))
        .collect()
}
