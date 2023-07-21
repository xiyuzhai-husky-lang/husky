use smallvec::SmallVec;

use super::*;

#[salsa::interned(db = EntityTreeDb, jar = EntityTreeJar, constructor = new_inner)]
pub struct TypeItemSynNodePath {
    maybe_ambiguous_path: MaybeAmbiguousPath<TypeItemPath>,
}

impl TypeItemSynNodePath {
    fn new(db: &dyn EntityTreeDb, registry: &mut EntityNodeRegistry, path: TypeItemPath) -> Self {
        Self::new_inner(db, registry.issue_maybe_ambiguous_path(path))
    }

    pub fn path(self, db: &dyn EntityTreeDb) -> Option<TypeItemPath> {
        self.maybe_ambiguous_path(db).unambiguous_path()
    }

    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        self.maybe_ambiguous_path(db).path.module_path(db)
    }

    pub fn impl_block(self, db: &dyn EntityTreeDb) -> TypeImplBlockSynNodePath {
        self.maybe_ambiguous_path(db)
            .path
            .impl_block(db)
            .node_path(db)
    }

    pub fn item_kind(self, db: &dyn EntityTreeDb) -> TypeItemKind {
        self.maybe_ambiguous_path(db).path.item_kind(db)
    }

    pub fn node(self, db: &dyn EntityTreeDb) -> TypeItemNode {
        ty_item_node(db, self)
    }
}

impl From<TypeItemSynNodePath> for EntitySynNodePath {
    fn from(id: TypeItemSynNodePath) -> Self {
        EntitySynNodePath::AssociatedItem(id.into())
    }
}

impl HasSynNodePath for TypeItemPath {
    type SynNodePath = TypeItemSynNodePath;

    fn node_path(self, db: &dyn EntityTreeDb) -> Self::SynNodePath {
        TypeItemSynNodePath::new_inner(db, MaybeAmbiguousPath::from_path(self))
    }
}

#[salsa::tracked(db = EntityTreeDb, jar = EntityTreeJar, constructor = new_inner)]
pub struct TypeItemNode {
    #[id]
    pub node_path: TypeItemSynNodePath,
    pub ast_idx: AstIdx,
    pub ident: Ident,
    pub item_kind: TypeItemKind,
    pub visibility: Scope,
    pub is_generic: bool,
}

impl TypeItemNode {
    #[inline(always)]
    fn new(
        db: &dyn EntityTreeDb,
        registry: &mut EntityNodeRegistry,
        impl_block_node_path: TypeImplBlockSynNodePath,
        ast_idx: AstIdx,
        ident: Ident,
        item_kind: TypeItemKind,
        visibility: Scope,
        is_generic: bool,
    ) -> (TypeItemSynNodePath, Self) {
        let path = TypeItemPath::new(db, impl_block_node_path.path(), ident, item_kind);
        let node_path = TypeItemSynNodePath::new(db, registry, path);
        (
            node_path,
            Self::new_inner(
                db, node_path, ast_idx, ident, item_kind, visibility, is_generic,
            ),
        )
    }

    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        self.node_path(db).module_path(db)
    }
}

#[salsa::tracked(jar = EntityTreeJar)]
pub(crate) fn ty_item_node(db: &dyn EntityTreeDb, node_path: TypeItemSynNodePath) -> TypeItemNode {
    node_path
        .impl_block(db)
        .items(db)
        .iter()
        .copied()
        .find_map(|(_, node_path1, node)| (node_path1 == node_path).then_some(node))
        .expect("some")
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn ty_impl_block_items(
    db: &dyn EntityTreeDb,
    node_path: TypeImplBlockSynNodePath,
) -> Vec<(Ident, TypeItemSynNodePath, TypeItemNode)> {
    let impl_block_node = node_path.node(db);
    let module_path = node_path.module_path(db);
    let ast_sheet = db.ast_sheet(module_path).unwrap();
    let items = impl_block_node.items(db);
    let mut registry = EntityNodeRegistry::default();
    items
        .ast_idx_range()
        .into_iter()
        .filter_map(|ast_idx| {
            let ast = &ast_sheet[ast_idx];
            match ast {
                Ast::Defn {
                    visibility_expr,
                    entity_kind,
                    ident_token,
                    is_generic,
                    ..
                } => {
                    let item_kind = match entity_kind {
                        EntityKind::AssociatedItem {
                            associated_item_kind: AssociatedItemKind::TypeItem(ty_item_kind),
                        } => *ty_item_kind,
                        _ => unreachable!(),
                    };
                    let (node_path, node) = TypeItemNode::new(
                        db,
                        &mut registry,
                        node_path,
                        ast_idx,
                        ident_token.ident(),
                        item_kind,
                        visibility_expr.visibility(),
                        *is_generic,
                    );
                    Some((ident_token.ident(), node_path, node))
                }
                Ast::Err { .. } => None,
                _ => unreachable!(),
            }
        })
        .collect()
}

pub trait HasItemNodePaths: Copy {
    type ItemNodePath;

    fn item_node_paths<'a>(
        self,
        db: &'a dyn EntityTreeDb,
    ) -> EntityTreeBundleResultRef<'a, &'a [(Ident, Self::ItemNodePath)]>;
}

impl HasItemNodePaths for TypePath {
    type ItemNodePath = TypeItemSynNodePath;

    fn item_node_paths<'a>(
        self,
        db: &'a dyn EntityTreeDb,
    ) -> EntityTreeBundleResultRef<'a, &'a [(Ident, TypeItemSynNodePath)]> {
        ty_item_node_paths(db, self).as_ref().map(|v| v as &[_])
    }
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn ty_item_node_paths(
    db: &dyn EntityTreeDb,
    path: TypePath,
) -> EntityTreeBundleResult<Vec<(Ident, TypeItemSynNodePath)>> {
    let crate_path = path.module_path(db).crate_path(db);
    let entity_tree_crate_bundle = db.entity_tree_bundle(crate_path)?;
    Ok(entity_tree_crate_bundle
        .all_ty_impl_block_node_paths()
        .filter_map(|node_path| {
            // ad hoc
            // todo: guard against two methods with the same ident
            (node_path.ty_path(db) == path).then(|| {
                node_path
                    .items(db)
                    .iter()
                    .copied()
                    .map(|(ident, node_path, node)| (ident, node_path))
            })
        })
        .flatten()
        .collect())
}

pub trait HasItemPathsMap: Copy {
    type ItemKind;

    type ItemPath;

    fn item_paths_map<'a>(
        self,
        db: &'a dyn EntityTreeDb,
    ) -> EntityTreeBundleResultRef<
        'a,
        &'a [(
            Ident,
            (
                Self::ItemKind,
                EntityTreeResult<SmallVec<[Self::ItemPath; 1]>>,
            ),
        )],
    >;
}

impl HasItemPathsMap for TypePath {
    type ItemKind = TypeItemKind;

    type ItemPath = TypeItemPath;

    fn item_paths_map<'a>(
        self,
        db: &'a dyn EntityTreeDb,
    ) -> EntityTreeBundleResultRef<
        'a,
        &'a [(
            Ident,
            (TypeItemKind, EntityTreeResult<SmallVec<[TypeItemPath; 1]>>),
        )],
    > {
        ty_item_paths_map(db, self).as_ref().map(|v| v as &[_])
    }
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn ty_item_paths_map(
    db: &dyn EntityTreeDb,
    path: TypePath,
) -> EntityTreeBundleResult<
    IdentPairMap<(TypeItemKind, EntityTreeResult<SmallVec<[TypeItemPath; 1]>>)>,
> {
    let mut paths: IdentPairMap<(TypeItemKind, EntityTreeResult<SmallVec<[TypeItemPath; 1]>>)> =
        Default::default();
    for (ident, node_path) in path.item_node_paths(db)?.iter().copied() {
        if let Some(path) = node_path.path(db) {
            let ty_item_kind = path.item_kind(db);
            paths.update_value_or_insert(
                ident,
                |(ty_item_kind0, result)| match result {
                    Ok(ref mut same_name_paths) => match *ty_item_kind0 == ty_item_kind {
                        true => todo!(),
                        false => todo!(),
                    },
                    Err(_) => (),
                },
                (ty_item_kind, Ok(smallvec::smallvec![path])),
            )
        }
    }
    Ok(paths)
}
