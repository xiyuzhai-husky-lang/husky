use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct TraitForTypeImplBlockNodePath {
    path: TraitForTypeImplBlockPath,
}

impl TraitForTypeImplBlockNodePath {
    pub fn path(self) -> TraitForTypeImplBlockPath {
        self.path
    }

    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        self.path.module_path(db)
    }

    pub fn trai_path(self, db: &dyn EntityTreeDb) -> TraitPath {
        self.path.trai_path(db)
    }

    pub fn ty_path(self, db: &dyn EntityTreeDb) -> TypePath {
        self.path.ty_path(db)
    }

    pub fn item_node_paths(self, db: &dyn EntityTreeDb) -> &[TraitForTypeItemNodePath] {
        todo!()
    }
}

impl From<TraitForTypeImplBlockNodePath> for EntityNodePath {
    fn from(id: TraitForTypeImplBlockNodePath) -> Self {
        EntityNodePath::ImplBlock(id.into())
    }
}

impl HasNodePath for TraitForTypeImplBlockPath {
    type NodePath = TraitForTypeImplBlockNodePath;

    fn node_path(self, db: &dyn EntityTreeDb) -> Self::NodePath {
        todo!()
    }
}

#[salsa::tracked(db = EntityTreeDb, jar = EntityTreeJar, constructor = new_inner)]
pub struct TraitForTypeImplBlockNode {
    #[id]
    pub node_path: TraitForTypeImplBlockNodePath,
    pub ast_idx: AstIdx,
    pub impl_token: ImplToken,
    pub trai_expr: ModuleItemPathExprIdx,
    pub for_token: TokenIdx,
    pub ty_expr: ModuleItemPathExprIdx,
    pub items: Option<ImplBlockItems>,
}

impl TraitForTypeImplBlockNode {
    pub(super) fn new(
        db: &dyn EntityTreeDb,
        registry: &mut ImplBlockRegistry,
        module_path: ModulePath,
        ast_idx: AstIdx,
        impl_token: ImplToken,
        trai_expr: ModuleItemPathExprIdx,
        trai_path: TraitPath,
        for_token: TokenIdx,
        ty_expr: ModuleItemPathExprIdx,
        ty_path: TypePath,
        items: Option<ImplBlockItems>,
    ) -> Self {
        TraitForTypeImplBlockNode::new_inner(
            db,
            TraitForTypeImplBlockNodePath {
                path: TraitForTypeImplBlockPath::new(db, registry, module_path, trai_path, ty_path),
            },
            ast_idx,
            impl_token,
            trai_expr,
            for_token,
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

    pub fn trai_path(self, db: &dyn EntityTreeDb) -> TraitPath {
        self.node_path(db).path.trai_path(db)
    }
}
