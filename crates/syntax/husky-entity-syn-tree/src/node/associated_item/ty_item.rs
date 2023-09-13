use smallvec::SmallVec;

use super::*;

#[salsa::interned(db = EntitySynTreeDb, jar = EntitySynTreeJar, constructor = new_inner)]
pub struct TypeItemSynNodePath {
    // no need for intro
    // ```
    // pub impl_block_syn_node_path: TypeImplBlockSynNodePath,
    // ```
    // because `impl_block_syn_node_path`s are not ambiguous
    maybe_ambiguous_path: MaybeAmbiguousPath<TypeItemPath>,
}

impl TypeItemSynNodePath {
    fn new(
        db: &dyn EntitySynTreeDb,
        impl_block_syn_node_path: TypeImplBlockSynNodePath,
        registry: &mut ItemSynNodePathRegistry,
        path: TypeItemPath,
    ) -> Self {
        Self::new_inner(db, registry.issue_maybe_ambiguous_path(path))
    }

    pub fn path(self, db: &dyn EntitySynTreeDb) -> Option<TypeItemPath> {
        self.maybe_ambiguous_path(db).unambiguous_path()
    }

    pub fn impl_block(self, db: &dyn EntitySynTreeDb) -> TypeImplBlockSynNodePath {
        self.maybe_ambiguous_path(db)
            .path
            .impl_block(db)
            .syn_node_path(db)
    }

    pub fn item_kind(self, db: &dyn EntitySynTreeDb) -> TypeItemKind {
        self.maybe_ambiguous_path(db).path.item_kind(db)
    }

    pub(crate) fn syn_node(self, db: &dyn EntitySynTreeDb) -> TypeItemSynNode {
        ty_item_syn_node(db, self)
    }
}

impl<Db> HasModulePath<Db> for TypeItemSynNodePath
where
    Db: ?Sized + EntitySynTreeDb,
{
    fn module_path(self, db: &Db) -> ModulePath {
        let db = entity_syn_tree_db(db);
        self.maybe_ambiguous_path(db).path.module_path(db)
    }
}

impl From<TypeItemSynNodePath> for ItemSynNodePath {
    fn from(id: TypeItemSynNodePath) -> Self {
        ItemSynNodePath::AssociatedItem(id.into())
    }
}

impl HasSynNodePath for TypeItemPath {
    type SynNodePath = TypeItemSynNodePath;

    fn syn_node_path(self, db: &dyn EntitySynTreeDb) -> Self::SynNodePath {
        TypeItemSynNodePath::new_inner(db, MaybeAmbiguousPath::from_path(self))
    }
}

#[salsa::tracked(db = EntitySynTreeDb, jar = EntitySynTreeJar, constructor = new_inner)]
pub(crate) struct TypeItemSynNode {
    #[id]
    pub syn_node_path: TypeItemSynNodePath,
    pub ast_idx: AstIdx,
    pub ident: Ident,
    pub item_kind: TypeItemKind,
    pub visibility: Scope,
    pub is_generic: bool,
}

impl TypeItemSynNode {
    #[inline(always)]
    fn new(
        db: &dyn EntitySynTreeDb,
        registry: &mut ItemSynNodePathRegistry,
        impl_block_syn_node_path: TypeImplBlockSynNodePath,
        ast_idx: AstIdx,
        ident: Ident,
        item_kind: TypeItemKind,
        visibility: Scope,
        is_generic: bool,
    ) -> (TypeItemSynNodePath, Self) {
        let path = TypeItemPath::new(db, impl_block_syn_node_path.path(), ident, item_kind);
        let syn_node_path = TypeItemSynNodePath::new(db, impl_block_syn_node_path, registry, path);
        (
            syn_node_path,
            Self::new_inner(
                db,
                syn_node_path,
                ast_idx,
                ident,
                item_kind,
                visibility,
                is_generic,
            ),
        )
    }

    pub fn module_path(self, db: &dyn EntitySynTreeDb) -> ModulePath {
        self.syn_node_path(db).module_path(db)
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
pub(crate) fn ty_item_syn_node(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TypeItemSynNodePath,
) -> TypeItemSynNode {
    syn_node_path
        .impl_block(db)
        .items(db)
        .iter()
        .copied()
        .find_map(|(_, node_path1, node)| (node_path1 == syn_node_path).then_some(node))
        .expect("some")
}

#[salsa::tracked(jar = EntitySynTreeJar, return_ref)]
pub(crate) fn ty_impl_block_items(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TypeImplBlockSynNodePath,
) -> Vec<(Ident, TypeItemSynNodePath, TypeItemSynNode)> {
    let impl_block_syn_node = syn_node_path.syn_node(db);
    let module_path = syn_node_path.module_path(db);
    let ast_sheet = db.ast_sheet(module_path).unwrap();
    let items = impl_block_syn_node.items(db);
    let mut registry = ItemSynNodePathRegistry::default();
    items
        .ast_idx_range()
        .into_iter()
        .filter_map(|ast_idx| {
            let ast = &ast_sheet[ast_idx];
            match ast {
                Ast::Identifiable {
                    visibility_expr,
                    item_kind,
                    ident_token,
                    is_generic,
                    ..
                } => {
                    let item_kind = match item_kind {
                        EntityKind::AssociatedItem {
                            associated_item_kind: AssociatedItemKind::TypeItem(ty_item_kind),
                        } => *ty_item_kind,
                        _ => unreachable!(),
                    };
                    let (syn_node_path, node) = TypeItemSynNode::new(
                        db,
                        &mut registry,
                        syn_node_path,
                        ast_idx,
                        ident_token.ident(),
                        item_kind,
                        visibility_expr.visibility(),
                        *is_generic,
                    );
                    Some((ident_token.ident(), syn_node_path, node))
                }
                Ast::Err { .. } => None,
                _ => unreachable!(),
            }
        })
        .collect()
}

pub trait HasItemNodePaths: Copy {
    type ItemNodePath;

    fn item_syn_node_paths<'a>(
        self,
        db: &'a dyn EntitySynTreeDb,
    ) -> EntityTreeBundleResultRef<'a, &'a [(Ident, Self::ItemNodePath)]>;
}

impl HasItemNodePaths for TypePath {
    type ItemNodePath = TypeItemSynNodePath;

    fn item_syn_node_paths<'a>(
        self,
        db: &'a dyn EntitySynTreeDb,
    ) -> EntityTreeBundleResultRef<'a, &'a [(Ident, TypeItemSynNodePath)]> {
        ty_item_syn_node_paths(db, self).as_ref().map(|v| v as &[_])
    }
}

#[salsa::tracked(jar = EntitySynTreeJar, return_ref)]
pub(crate) fn ty_item_syn_node_paths(
    db: &dyn EntitySynTreeDb,
    path: TypePath,
) -> EntitySynTreeBundleResult<Vec<(Ident, TypeItemSynNodePath)>> {
    let crate_path = path.module_path(db).crate_path(db);
    let item_tree_crate_bundle = db.item_syn_tree_bundle(crate_path)?;
    Ok(item_tree_crate_bundle
        .all_ty_impl_block_syn_node_paths()
        .filter_map(|syn_node_path| {
            // ad hoc
            // todo: guard against two methods with the same ident
            (syn_node_path.ty_path(db) == path).then(|| {
                syn_node_path
                    .items(db)
                    .iter()
                    .copied()
                    .map(|(ident, syn_node_path, node)| (ident, syn_node_path))
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
        db: &'a dyn EntitySynTreeDb,
    ) -> EntityTreeBundleResultRef<
        'a,
        &'a [(
            Ident,
            (
                Self::ItemKind,
                EntitySynTreeResult<SmallVec<[Self::ItemPath; 1]>>,
            ),
        )],
    >;
}

impl HasItemPathsMap for TypePath {
    type ItemKind = TypeItemKind;

    type ItemPath = TypeItemPath;

    fn item_paths_map<'a>(
        self,
        db: &'a dyn EntitySynTreeDb,
    ) -> EntityTreeBundleResultRef<
        'a,
        &'a [(
            Ident,
            (
                TypeItemKind,
                EntitySynTreeResult<SmallVec<[TypeItemPath; 1]>>,
            ),
        )],
    > {
        ty_item_paths_map(db, self).as_ref().map(|v| v as &[_])
    }
}

#[salsa::tracked(jar = EntitySynTreeJar, return_ref)]
pub(crate) fn ty_item_paths_map(
    db: &dyn EntitySynTreeDb,
    path: TypePath,
) -> EntitySynTreeBundleResult<
    IdentPairMap<(
        TypeItemKind,
        EntitySynTreeResult<SmallVec<[TypeItemPath; 1]>>,
    )>,
> {
    let mut paths: IdentPairMap<(
        TypeItemKind,
        EntitySynTreeResult<SmallVec<[TypeItemPath; 1]>>,
    )> = Default::default();
    for (ident, syn_node_path) in path.item_syn_node_paths(db)?.iter().copied() {
        if let Some(path) = syn_node_path.path(db) {
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
