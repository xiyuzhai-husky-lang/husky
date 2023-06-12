use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct TypeImplBlockNodePath {
    path: TypeImplBlockPath,
}

impl TypeImplBlockNodePath {
    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        self.path.module_path(db)
    }

    pub fn ty_path(self, db: &dyn EntityTreeDb) -> TypePath {
        self.path.ty_path(db)
    }
}

impl From<TypeImplBlockNodePath> for EntityNodePath {
    fn from(id: TypeImplBlockNodePath) -> Self {
        EntityNodePath::ImplBlock(id.into())
    }
}

#[salsa::tracked(db = EntityTreeDb, jar = EntityTreeJar, constructor = new_inner)]
pub struct TypeImplBlockNode {
    #[id]
    pub id: TypeImplBlockNodePath,
    pub ast_idx: AstIdx,
    pub impl_token: ImplToken,
    pub ty_expr: ModuleItemPathExprIdx,
    pub body: ImplBlockItems,
}

impl TypeImplBlockNode {
    pub(super) fn new(
        db: &dyn EntityTreeDb,
        impl_token: ImplToken,
        registry: &mut ImplBlockRegistry,
        module_path: ModulePath,
        ast_idx: AstIdx,
        body: ImplBlockItems,
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
            body,
        )
    }

    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        self.id(db).path.module_path(db)
    }

    pub fn ty_path(self, db: &dyn EntityTreeDb) -> TypePath {
        self.id(db).path.ty_path(db)
    }

    pub fn items(self, db: &dyn EntityTreeDb) -> Vec<(Ident, AssociatedItemNode)> {
        calc_impl_block_items(db, self.into(), self.module_path(db), self.body(db))
    }
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn ty_impl_block_items(
    db: &dyn EntityTreeDb,
    impl_block: TypeImplBlockNode,
) -> Vec<(Ident, TypeItemNode)> {
    todo!()
    // calc_impl_block_items(
    //     db,
    //     impl_block.into(),
    //     impl_block.module_path(db),
    //     impl_block.body(db),
    // )
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct TypeImplBlockId {
    module_path: ModulePath,
    ty_path: TypePath,
    disambiguator: u8,
}

impl TypeImplBlockId {
    pub fn module_path(&self) -> ModulePath {
        self.module_path
    }

    pub fn ty_path(&self) -> TypePath {
        self.ty_path
    }

    pub fn disambiguator(&self) -> u8 {
        self.disambiguator
    }
}
