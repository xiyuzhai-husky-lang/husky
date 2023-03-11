use super::*;

#[salsa::tracked(db = EntityTreeDb, jar = EntityTreeJar, constructor = new_inner)]
pub struct TypeAsTraitImplBlock {
    #[id]
    pub id: TypeAsTraitImplBlockId,
    pub ast_idx: AstIdx,
    pub body: AstIdxRange,
}

impl TypeAsTraitImplBlock {
    pub(super) fn new(
        db: &dyn EntityTreeDb,
        registry: &mut ImplBlockRegistry,
        module_path: ModulePath,
        ast_idx: AstIdx,
        body: AstIdxRange,
        ty: TypePath,
        trai: TraitPath,
    ) -> Self {
        todo!()
    }

    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        self.id(db).module_path
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct TypeAsTraitImplBlockId {
    module_path: ModulePath,
    ty: TypePath,
    trai: TraitPath,
    disambiguator: u8,
}

impl TypeAsTraitImplBlockId {
    pub fn module_path(&self) -> ModulePath {
        self.module_path
    }

    pub fn ty(&self) -> TypePath {
        self.ty
    }

    pub fn trai(&self) -> TraitPath {
        self.trai
    }

    pub fn disambiguator(&self) -> u8 {
        self.disambiguator
    }
}
