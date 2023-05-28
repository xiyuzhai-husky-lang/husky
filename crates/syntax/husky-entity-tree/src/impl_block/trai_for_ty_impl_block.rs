use super::*;

#[salsa::tracked(db = EntityTreeDb, jar = EntityTreeJar, constructor = new_inner)]
pub struct TraitForTypeImplBlock {
    #[id]
    pub id: TraitForTypeImplBlockId,
    pub ast_idx: AstIdx,
    pub impl_token: ImplToken,
    pub trai_expr: MajorPathExprIdx,
    pub for_token: TokenIdx,
    pub ty_expr: MajorPathExprIdx,
    pub items: Option<ImplBlockItems>,
}

impl TraitForTypeImplBlock {
    pub(super) fn new(
        db: &dyn EntityTreeDb,
        registry: &mut ImplBlockRegistry,
        module_path: ModulePath,
        ast_idx: AstIdx,
        impl_token: ImplToken,
        trai_expr: MajorPathExprIdx,
        trai_path: TraitPath,
        for_token: TokenIdx,
        ty_expr: MajorPathExprIdx,
        ty_path: TypePath,
        items: Option<ImplBlockItems>,
    ) -> Self {
        TraitForTypeImplBlock::new_inner(
            db,
            TraitForTypeImplBlockId {
                module_path,
                trai_path,
                ty_path,
                disambiguator: registry.issue_disambiguitor(
                    module_path,
                    ImplBlockKind::TraitForType { ty_path, trai_path },
                ),
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
        self.id(db).module_path
    }

    pub fn ty(self, db: &dyn EntityTreeDb) -> TypePath {
        self.id(db).ty_path
    }

    pub fn trai(self, db: &dyn EntityTreeDb) -> TraitPath {
        self.id(db).trai_path
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct TraitForTypeImplBlockId {
    module_path: ModulePath,
    trai_path: TraitPath,
    ty_path: TypePath,
    disambiguator: u8,
}

impl TraitForTypeImplBlockId {
    pub fn module(&self) -> ModulePath {
        self.module_path
    }

    pub fn ty_path(&self) -> TypePath {
        self.ty_path
    }

    pub fn trai_path(&self) -> TraitPath {
        self.trai_path
    }

    pub fn disambiguator(&self) -> u8 {
        self.disambiguator
    }
}
