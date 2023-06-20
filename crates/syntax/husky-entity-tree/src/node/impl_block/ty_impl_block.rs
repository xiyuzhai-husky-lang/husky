use husky_entity_taxonomy::AssociatedItemKind;

use super::*;

// basically a wrapper type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct TypeImplBlockNodePath {
    path: TypeImplBlockPath,
}

impl salsa::AsId for TypeImplBlockNodePath {
    fn as_id(self) -> salsa::Id {
        self.path.as_id()
    }

    fn from_id(id: salsa::Id) -> Self {
        TypeImplBlockNodePath {
            path: TypeImplBlockPath::from_id(id),
        }
    }
}

impl<DB> salsa::salsa_struct::SalsaStructInDb<DB> for TypeImplBlockNodePath
where
    DB: ?Sized + salsa::DbWithJar<EntityPathJar>,
{
    fn register_dependent_fn(_db: &DB, _index: salsa::routes::IngredientIndex) {}
}

impl From<TypeImplBlockNodePath> for EntityNodePath {
    #[inline(always)]
    fn from(id: TypeImplBlockNodePath) -> Self {
        EntityNodePath::ImplBlock(id.into())
    }
}

impl TypeImplBlockNodePath {
    #[inline(always)]
    pub fn path(self) -> TypeImplBlockPath {
        self.path
    }

    #[inline(always)]
    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        self.path.module_path(db)
    }

    #[inline(always)]
    pub fn ty_path(self, db: &dyn EntityTreeDb) -> TypePath {
        self.path.ty_path(db)
    }

    #[inline(always)]
    pub fn node(self, db: &dyn EntityTreeDb) -> TypeImplBlockNode {
        ty_impl_block_node(db, self)
    }

    #[inline(always)]
    pub fn items(self, db: &dyn EntityTreeDb) -> &[(Ident, TypeItemNodePath, TypeItemNode)] {
        ty_impl_block_items(db, self)
    }

    #[inline(always)]
    pub fn item_node_paths<'a>(
        self,
        db: &'a dyn EntityTreeDb,
    ) -> impl Iterator<Item = TypeItemNodePath> + 'a {
        self.items(db)
            .iter()
            .copied()
            .map(|(_, node_path, _)| node_path)
    }
}

impl HasNodePath for TypeImplBlockPath {
    type NodePath = TypeImplBlockNodePath;

    #[inline(always)]
    fn node_path(self, db: &dyn EntityTreeDb) -> Self::NodePath {
        TypeImplBlockNodePath { path: self }
    }
}

#[salsa::tracked(db = EntityTreeDb, jar = EntityTreeJar, constructor = new_inner)]
pub struct TypeImplBlockNode {
    #[id]
    pub node_path: TypeImplBlockNodePath,
    pub ast_idx: AstIdx,
    pub impl_token: ImplToken,
    pub ty_expr: ModuleItemPathExprIdx,
    pub items: TypeItems,
}

impl TypeImplBlockNode {
    pub(super) fn new(
        db: &dyn EntityTreeDb,
        impl_token: ImplToken,
        registry: &mut ImplBlockRegistry,
        module_path: ModulePath,
        ast_idx: AstIdx,
        items: TypeItems,
        ty_path: TypePath,
        ty_expr: ModuleItemPathExprIdx,
    ) -> Self {
        Self::new_inner(
            db,
            TypeImplBlockNodePath {
                path: TypeImplBlockPath::new(db, registry, module_path, ty_path),
            },
            ast_idx,
            impl_token,
            ty_expr,
            items,
        )
    }

    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        self.node_path(db).path.module_path(db)
    }

    pub fn ty_path(self, db: &dyn EntityTreeDb) -> TypePath {
        self.node_path(db).path.ty_path(db)
    }
}

#[salsa::tracked(jar = EntityTreeJar)]
pub(crate) fn ty_impl_block_node(
    db: &dyn EntityTreeDb,
    node_path: TypeImplBlockNodePath,
) -> TypeImplBlockNode {
    let module_path = node_path.module_path(db);
    let entity_tree_sheet = db.entity_tree_sheet(module_path).expect("valid module");
    entity_tree_sheet.ty_impl_block_node(node_path)
}
