use super::*;
use husky_entity_taxonomy::AssociatedItemKind;

// basically a wrapper type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = EntitySynTreeDb)]
pub struct TypeImplBlockSynNodePath {
    path: TypeImplBlockPath,
}

impl salsa::AsId for TypeImplBlockSynNodePath {
    fn as_id(self) -> salsa::Id {
        self.path.as_id()
    }

    fn from_id(id: salsa::Id) -> Self {
        TypeImplBlockSynNodePath {
            path: TypeImplBlockPath::from_id(id),
        }
    }
}

impl<DB> salsa::salsa_struct::SalsaStructInDb<DB> for TypeImplBlockSynNodePath
where
    DB: ?Sized + salsa::DbWithJar<EntityPathJar>,
{
    fn register_dependent_fn(_db: &DB, _index: salsa::routes::IngredientIndex) {}
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
    pub fn module_path(self, db: &dyn EntitySynTreeDb) -> ModulePath {
        self.path.module_path(db)
    }

    #[inline(always)]
    pub fn ty_path(self, db: &dyn EntitySynTreeDb) -> TypePath {
        self.path.ty_path(db)
    }

    #[inline(always)]
    pub fn node(self, db: &dyn EntitySynTreeDb) -> TypeImplBlockSynNode {
        ty_impl_block_syn_node(db, self)
    }

    #[inline(always)]
    pub fn items(
        self,
        db: &dyn EntitySynTreeDb,
    ) -> &[(Ident, TypeItemSynNodePath, TypeItemSynNode)] {
        ty_impl_block_items(db, self)
    }

    #[inline(always)]
    pub fn item_syn_node_paths<'a>(
        self,
        db: &'a dyn EntitySynTreeDb,
    ) -> impl Iterator<Item = TypeItemSynNodePath> + 'a {
        self.items(db)
            .iter()
            .copied()
            .map(|(_, syn_node_path, _)| syn_node_path)
    }
}

impl HasSynNodePath for TypeImplBlockPath {
    type SynNodePath = TypeImplBlockSynNodePath;

    #[inline(always)]
    fn syn_node_path(self, db: &dyn EntitySynTreeDb) -> Self::SynNodePath {
        TypeImplBlockSynNodePath { path: self }
    }
}

#[salsa::tracked(db = EntitySynTreeDb, jar = EntitySynTreeJar, constructor = new_inner)]
pub struct TypeImplBlockSynNode {
    #[id]
    pub syn_node_path: TypeImplBlockSynNodePath,
    pub ast_idx: AstIdx,
    pub impl_token: ImplToken,
    pub ty_expr: MajorItemPathExprIdx,
    pub items: TypeItems,
}

impl TypeImplBlockSynNode {
    pub(super) fn new(
        db: &dyn EntitySynTreeDb,
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
            TypeImplBlockSynNodePath {
                path: TypeImplBlockPath::new(db, registry, module_path, ty_path),
            },
            ast_idx,
            impl_token,
            ty_expr,
            items,
        )
    }

    pub fn module_path(self, db: &dyn EntitySynTreeDb) -> ModulePath {
        self.syn_node_path(db).path.module_path(db)
    }

    pub fn ty_path(self, db: &dyn EntitySynTreeDb) -> TypePath {
        self.syn_node_path(db).path.ty_path(db)
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
pub(crate) fn ty_impl_block_syn_node(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TypeImplBlockSynNodePath,
) -> TypeImplBlockSynNode {
    let module_path = syn_node_path.module_path(db);
    let item_tree_sheet = db.item_syn_tree_sheet(module_path).expect("valid module");
    item_tree_sheet.ty_impl_block_syn_node(syn_node_path)
}
