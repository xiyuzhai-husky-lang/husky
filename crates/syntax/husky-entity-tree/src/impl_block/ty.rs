use super::*;

#[salsa::tracked(db = EntityTreeDb, jar = EntityTreeJar, constructor = new_inner)]
pub struct TypeImplBlock {
    #[id]
    pub id: TypeImplBlockId,
    pub ast_idx: AstIdx,
    pub body: AstIdxRange,
}

impl TypeImplBlock {
    pub(super) fn new(
        db: &dyn EntityTreeDb,
        registry: &mut ImplBlockRegistry,
        module_path: ModulePath,
        ast_idx: AstIdx,
        body: AstIdxRange,
        ty: TypePath,
    ) -> Self {
        Self::new_inner(
            db,
            TypeImplBlockId {
                module: module_path,
                ty,
                disambiguator: registry
                    .issue_disambiguitor(module_path, ImplBlockKind::Type { ty }),
            },
            ast_idx,
            body,
        )
    }

    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        self.id(db).module
    }

    pub fn ty(self, db: &dyn EntityTreeDb) -> TypePath {
        self.id(db).ty
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct TypeImplBlockId {
    module: ModulePath,
    ty: TypePath,
    disambiguator: u8,
}

impl TypeImplBlockId {
    pub fn module(&self) -> ModulePath {
        self.module
    }

    pub fn ty(&self) -> TypePath {
        self.ty
    }

    pub fn disambiguator(&self) -> u8 {
        self.disambiguator
    }
}
