use super::*;

#[salsa::tracked(db = EntityTreeDb, jar = EntityTreeJar, constructor = new_inner)]
pub struct TypeImplBlock {
    #[id]
    pub id: TypeImplBlockId,
    pub ast_idx: AstIdx,
    pub impl_token: ImplToken,
    pub ty_expr: ModuleItemPathExprIdx,
    pub body: ImplBlockItems,
}

impl TypeImplBlock {
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
            TypeImplBlockId {
                module_path,
                ty_path,
                disambiguator: registry
                    .issue_disambiguitor(module_path, ImplBlockKind::Type { ty_path }),
            },
            ast_idx,
            impl_token,
            ty_expr,
            body,
        )
    }

    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        self.id(db).module_path
    }

    pub fn ty_path(self, db: &dyn EntityTreeDb) -> TypePath {
        self.id(db).ty_path
    }
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
