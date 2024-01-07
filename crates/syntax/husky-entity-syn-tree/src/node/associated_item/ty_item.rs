use smallvec::SmallVec;

use super::*;

#[salsa::debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::as_id]
#[salsa::deref_id]
pub struct TypeItemSynNodePath(ItemSynNodePathId);

#[salsa::debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeItemSynNodePathData {
    // no need for intro
    // ```
    // pub impl_block_syn_node_path: TypeImplBlockSynNodePath,
    // ```
    // because `impl_block_syn_node_path`s are not ambiguous
    maybe_ambiguous_path: MaybeAmbiguousPath<TypeItemPath>,
}

impl TypeItemSynNodePath {
    fn new(
        db: &::salsa::Db,
        _impl_block_syn_node_path: TypeImplBlockSynNodePath,
        registry: &mut ItemSynNodePathRegistry,
        path: TypeItemPath,
    ) -> Self {
        Self(ItemSynNodePathId::new(
            db,
            ItemSynNodePathData::AssociatedItem(AssociatedItemSynNodePathData::TypeItem(
                TypeItemSynNodePathData {
                    maybe_ambiguous_path: registry.issue_maybe_ambiguous_path(path),
                },
            )),
        ))
    }

    pub(crate) fn syn_node<'a>(self, db: &'a ::salsa::Db) -> &'a TypeItemSynNode {
        self.data(db)
            .impl_block(db)
            .associated_items(db)
            .iter()
            .find_map(|&(_, node_path1, ref node)| (node_path1 == self).then_some(node))
            .expect("some")
    }

    pub fn data(self, db: &::salsa::Db) -> TypeItemSynNodePathData {
        match self.0.data(db) {
            ItemSynNodePathData::AssociatedItem(AssociatedItemSynNodePathData::TypeItem(data)) => {
                data
            }
            _ => unreachable!(),
        }
    }

    pub fn path(self, db: &::salsa::Db) -> Option<TypeItemPath> {
        Some(match self.0.path(db)? {
            ItemPath::AssociatedItem(AssociatedItemPath::TypeItem(path)) => path,
            _ => unreachable!(),
        })
    }
}

impl TypeItemSynNodePathData {
    #[inline(always)]
    pub fn syn_node_path(self, id: ItemSynNodePathId) -> TypeItemSynNodePath {
        TypeItemSynNodePath(id)
    }

    pub fn path(&self) -> Option<TypeItemPath> {
        self.maybe_ambiguous_path.unambiguous_path()
    }

    pub fn module_path(self, db: &::salsa::Db) -> ModulePath {
        self.maybe_ambiguous_path.path.module_path(db)
    }

    pub fn ast_idx(self, id: ItemSynNodePathId, db: &::salsa::Db) -> AstIdx {
        TypeItemSynNodePath(id).syn_node(db).ast_idx
    }

    pub fn impl_block(&self, db: &::salsa::Db) -> TypeImplBlockSynNodePath {
        self.maybe_ambiguous_path
            .path
            .impl_block(db)
            .syn_node_path(db)
    }

    pub fn item_kind(&self, db: &::salsa::Db) -> TypeItemKind {
        self.maybe_ambiguous_path.path.item_kind(db)
    }
}

impl From<TypeItemSynNodePath> for ItemSynNodePath {
    fn from(id: TypeItemSynNodePath) -> Self {
        ItemSynNodePath::AssociatedItem(id.into())
    }
}

impl HasSynNodePath for TypeItemPath {
    type SynNodePath = TypeItemSynNodePath;

    fn syn_node_path(self, db: &::salsa::Db) -> Self::SynNodePath {
        TypeItemSynNodePath(ItemSynNodePathId::new(
            db,
            ItemSynNodePathData::AssociatedItem(AssociatedItemSynNodePathData::TypeItem(
                TypeItemSynNodePathData {
                    maybe_ambiguous_path: MaybeAmbiguousPath::from_path(self),
                },
            )),
        ))
    }
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct TypeItemSynNode {
    pub(crate) syn_node_path: TypeItemSynNodePath,
    pub(crate) ast_idx: AstIdx,
    pub(crate) ident: Ident,
    pub(crate) item_kind: TypeItemKind,
    pub(crate) visibility: Scope,
    pub(crate) is_generic: bool,
}

impl TypeItemSynNode {
    #[inline(always)]
    fn new(
        db: &::salsa::Db,
        registry: &mut ItemSynNodePathRegistry,
        impl_block_syn_node_path: TypeImplBlockSynNodePath,
        ast_idx: AstIdx,
        ident: Ident,
        item_kind: TypeItemKind,
        visibility: Scope,
        is_generic: bool,
    ) -> (TypeItemSynNodePath, Self) {
        let path = TypeItemPath::new(impl_block_syn_node_path.path(db), ident, item_kind, db);
        let syn_node_path = TypeItemSynNodePath::new(db, impl_block_syn_node_path, registry, path);
        (
            syn_node_path,
            Self {
                syn_node_path,
                ast_idx,
                ident,
                item_kind,
                visibility,
                is_generic,
            },
        )
    }
}

#[salsa::tracked(jar = EntitySynTreeJar, return_ref)]
pub(crate) fn ty_impl_block_items(
    db: &::salsa::Db,
    syn_node_path: TypeImplBlockSynNodePath,
) -> Vec<(Ident, TypeItemSynNodePath, TypeItemSynNode)> {
    let impl_block_syn_node = syn_node_path.syn_node(db);
    let module_path = syn_node_path.module_path(db);
    let ast_sheet = module_path.ast_sheet(db);
    let items = impl_block_syn_node.items;
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

    fn item_syn_node_paths<'a>(self, db: &'a ::salsa::Db) -> &'a [(Ident, Self::ItemNodePath)];
}

impl HasItemNodePaths for TypePath {
    type ItemNodePath = TypeItemSynNodePath;

    fn item_syn_node_paths<'a>(self, db: &'a ::salsa::Db) -> &'a [(Ident, TypeItemSynNodePath)] {
        ty_item_syn_node_paths(db, self)
    }
}

#[salsa::tracked(jar = EntitySynTreeJar, return_ref)]
pub(crate) fn ty_item_syn_node_paths(
    db: &::salsa::Db,
    path: TypePath,
) -> Vec<(Ident, TypeItemSynNodePath)> {
    let crate_path = path.module_path(db).crate_path(db);
    let item_tree_crate_bundle = db.item_syn_tree_bundle(crate_path);
    item_tree_crate_bundle
        .all_ty_impl_block_syn_node_paths()
        .filter_map(|syn_node_path| {
            // ad hoc
            // todo: guard against two methods with the same ident
            (syn_node_path.ty_path(db) == path).then(|| {
                syn_node_path
                    .associated_items(db)
                    .iter()
                    .map(|&(ident, syn_node_path, _)| (ident, syn_node_path))
            })
        })
        .flatten()
        .collect()
}

pub trait HasItemPathsMap: Copy {
    type ItemKind;

    type ItemPath;

    fn item_paths_map<'a>(
        self,
        db: &'a ::salsa::Db,
    ) -> &'a [(
        Ident,
        (
            Self::ItemKind,
            EntitySynTreeResult<SmallVec<[Self::ItemPath; 1]>>,
        ),
    )];
}

impl HasItemPathsMap for TypePath {
    type ItemKind = TypeItemKind;

    type ItemPath = TypeItemPath;

    fn item_paths_map<'a>(
        self,
        db: &'a ::salsa::Db,
    ) -> &'a [(
        Ident,
        (
            TypeItemKind,
            EntitySynTreeResult<SmallVec<[TypeItemPath; 1]>>,
        ),
    )] {
        ty_item_paths_map(db, self)
    }
}

#[salsa::tracked(jar = EntitySynTreeJar, return_ref)]
pub(crate) fn ty_item_paths_map(
    db: &::salsa::Db,
    path: TypePath,
) -> IdentPairMap<(
    TypeItemKind,
    EntitySynTreeResult<SmallVec<[TypeItemPath; 1]>>,
)> {
    let mut paths: IdentPairMap<(
        TypeItemKind,
        EntitySynTreeResult<SmallVec<[TypeItemPath; 1]>>,
    )> = Default::default();
    for (ident, syn_node_path) in path.item_syn_node_paths(db).iter().copied() {
        if let Some(path) = syn_node_path.path(db) {
            let ty_item_kind = path.item_kind(db);
            paths.update_value_or_insert(
                ident,
                |(ty_item_kind0, result)| match result {
                    Ok(ref mut _same_name_paths) => match *ty_item_kind0 == ty_item_kind {
                        true => todo!(),
                        false => todo!(),
                    },
                    Err(_) => (),
                },
                (ty_item_kind, Ok(smallvec::smallvec![path])),
            )
        }
    }
    paths
}
