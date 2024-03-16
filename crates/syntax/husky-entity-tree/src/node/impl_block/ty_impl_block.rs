use super::*;

use vec_like::SmallVecPairMap;

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::as_id(jar = EntityTreeJar)]
#[salsa::deref_id]
pub struct TypeImplBlockSynNodePath(ItemSynNodePathId);

// basically a wrapper type
#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeImplBlockSynNodePathData {
    pub(crate) path: TypeImplBlockPath,
}

impl From<TypeImplBlockSynNodePath> for ItemSynNodePath {
    #[inline(always)]
    fn from(id: TypeImplBlockSynNodePath) -> Self {
        ItemSynNodePath::ImplBlock(id.into())
    }
}

impl TypeImplBlockSynNodePath {
    pub fn data(self, db: &::salsa::Db) -> TypeImplBlockSynNodePathData {
        match self.0.data(db) {
            ItemSynNodePathData::ImplBlock(ImplBlockSynNodePathData::TypeImplBlock(data)) => data,
            _ => unreachable!(),
        }
    }

    pub fn path(self, db: &::salsa::Db) -> TypeImplBlockPath {
        self.data(db).path
    }

    pub fn ty_path(self, db: &::salsa::Db) -> TypePath {
        self.path(db).ty_path(db)
    }

    pub(crate) fn syn_node<'a>(self, db: &'a ::salsa::Db) -> &'a TypeImplBlockSynNode {
        let module_path = self.module_path(db);
        let item_tree_sheet = db.item_syn_tree_sheet(module_path);
        item_tree_sheet.ty_impl_block_syn_node(self)
    }

    pub(crate) fn assoc_items(
        self,
        db: &::salsa::Db,
    ) -> &[(Ident, TypeItemSynNodePath, TypeItemSynNode)] {
        ty_impl_block_items(db, self)
    }

    #[inline(always)]
    pub fn item_syn_node_paths<'a>(
        self,
        db: &'a ::salsa::Db,
    ) -> impl Iterator<Item = TypeItemSynNodePath> + 'a {
        self.assoc_items(db)
            .iter()
            .map(|&(_, syn_node_path, _)| syn_node_path)
    }
}

impl TypeImplBlockSynNodePathData {
    #[inline(always)]
    pub fn syn_node_path(self, id: ItemSynNodePathId) -> TypeImplBlockSynNodePath {
        TypeImplBlockSynNodePath(id)
    }

    pub fn path(self) -> TypeImplBlockPath {
        self.path
    }

    pub fn module_path(self, db: &::salsa::Db) -> ModulePath {
        self.path.module_path(db)
    }

    pub fn ast_idx(self, id: ItemSynNodePathId, db: &::salsa::Db) -> AstIdx {
        TypeImplBlockSynNodePath(id).syn_node(db).ast_idx
    }
}

impl HasSynNodePath for TypeImplBlockPath {
    type SynNodePath = TypeImplBlockSynNodePath;

    #[inline(always)]
    fn syn_node_path(self, db: &::salsa::Db) -> Self::SynNodePath {
        TypeImplBlockSynNodePath(ItemSynNodePathId::new(
            db,
            ItemSynNodePathData::ImplBlock(ImplBlockSynNodePathData::TypeImplBlock(
                TypeImplBlockSynNodePathData { path: self },
            )),
        ))
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct TypeImplBlockSynNode {
    syn_node_path: TypeImplBlockSynNodePath,
    ast_idx: AstIdx,
    impl_token: ImplToken,
    ty_expr: MajorItemPathExprIdx,
    items: TypeItems,
}

/// # constructor
impl TypeImplBlockSynNode {
    pub(super) fn new(
        db: &::salsa::Db,
        impl_token: ImplToken,
        registry: &mut ImplBlockRegistry,
        module_path: ModulePath,
        ast_idx: AstIdx,
        items: TypeItems,
        ty_path: TypePath,
        ty_expr: MajorItemPathExprIdx,
    ) -> Self {
        Self {
            syn_node_path: TypeImplBlockSynNodePath(ItemSynNodePathId::new(
                db,
                ItemSynNodePathData::ImplBlock(ImplBlockSynNodePathData::TypeImplBlock(
                    TypeImplBlockSynNodePathData {
                        path: TypeImplBlockPath::new(db, registry, module_path, ty_path),
                    },
                )),
            )),
            ast_idx,
            impl_token,
            ty_expr,
            items,
        }
    }
}

/// # getters
impl TypeImplBlockSynNode {
    pub fn syn_node_path(&self) -> TypeImplBlockSynNodePath {
        self.syn_node_path
    }

    pub fn items(&self) -> TypeItems {
        self.items
    }
}

impl HasAssocItemPaths for TypeImplBlockPath {
    type AssocItemPath = TypeItemPath;

    fn assoc_item_paths(self, db: &::salsa::Db) -> &[(Ident, Self::AssocItemPath)] {
        ty_impl_block_item_paths(db, self)
    }
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
fn ty_impl_block_item_paths(
    db: &::salsa::Db,
    path: TypeImplBlockPath,
) -> SmallVecPairMap<Ident, TypeItemPath, 2> {
    path.syn_node_path(db)
        .assoc_items(db)
        .iter()
        .filter_map(|(ident, syn_node_path, _)| Some((*ident, syn_node_path.path(db)?)))
        .collect()
}
