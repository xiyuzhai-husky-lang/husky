use super::*;

use vec_like::SmallVecPairMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeImplBlockSynNodePath(ItemSynNodePathId);

// basically a wrapper type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeImplBlockSynNodePathData {
    path: TypeImplBlockPath,
}

impl From<TypeImplBlockSynNodePath> for ItemSynNodePath {
    #[inline(always)]
    fn from(id: TypeImplBlockSynNodePath) -> Self {
        ItemSynNodePath::ImplBlock(id.into())
    }
}

impl TypeImplBlockSynNodePath {
    #[inline(always)]
    pub fn path(self) -> TypeImplBlockPath {
        self.path
    }

    #[inline(always)]
    pub fn ty_path(self, db: &::salsa::Db) -> TypePath {
        self.path.ty_path(db)
    }

    #[inline(always)]
    pub(crate) fn syn_node(self, db: &::salsa::Db) -> TypeImplBlockSynNodeData {
        ty_impl_block_syn_node(db, self)
    }

    #[inline(always)]
    pub(crate) fn associated_items(
        self,
        db: &::salsa::Db,
    ) -> &[(Ident, TypeItemSynNodePath, TypeItemSynNodeData)] {
        ty_impl_block_items(db, self)
    }

    #[inline(always)]
    pub fn item_syn_node_paths<'a>(
        self,
        db: &'a ::salsa::Db,
    ) -> impl Iterator<Item = TypeItemSynNodePath> + 'a {
        self.associated_items(db)
            .iter()
            .copied()
            .map(|(_, syn_node_path, _)| syn_node_path)
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

pub(crate) struct TypeImplBlockSynNodeData {
    syn_node_path: TypeImplBlockSynNodePath,
    ast_idx: AstIdx,
    impl_regional_token: ImplToken,
    ty_expr: MajorItemPathExprIdx,
    items: TypeItems,
}

impl TypeImplBlockSynNodeData {
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
        Self::new_inner(
            db,
            TypeImplBlockSynNodePath(ItemSynNodePathId::new(
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
        )
    }

    pub fn module_path(self, db: &::salsa::Db) -> ModulePath {
        self.syn_node_path(db).path.module_path(db)
    }

    pub fn ty_path(self, db: &::salsa::Db) -> TypePath {
        self.syn_node_path(db).path.ty_path(db)
    }
}

pub(crate) fn ty_impl_block_syn_node(
    db: &::salsa::Db,
    syn_node_path: TypeImplBlockSynNodePath,
) -> TypeImplBlockSynNodeData {
    let module_path = todo!(); //syn_node_path.module_path(db);
    let item_tree_sheet = db.item_syn_tree_sheet(module_path);
    item_tree_sheet.ty_impl_block_syn_node(syn_node_path)
}

impl HasAssociatedItemPaths for TypeImplBlockPath {
    type AssociatedItemPath = TypeItemPath;

    fn associated_item_paths(self, db: &::salsa::Db) -> &[(Ident, Self::AssociatedItemPath)] {
        ty_impl_block_item_paths(db, self)
    }
}

#[salsa::tracked(jar = EntitySynTreeJar, return_ref)]
fn ty_impl_block_item_paths(
    db: &::salsa::Db,
    path: TypeImplBlockPath,
) -> SmallVecPairMap<Ident, TypeItemPath, 2> {
    path.syn_node_path(db)
        .associated_items(db)
        .iter()
        .filter_map(|(ident, syn_node_path, _)| Some((*ident, syn_node_path.path(db)?)))
        .collect()
}
