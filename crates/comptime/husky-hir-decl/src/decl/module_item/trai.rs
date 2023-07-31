use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct TraitHirDecl {
    pub path: TraitPath,
    #[return_ref]
    pub template_parameters: HirTemplateParameters,
    pub hir_expr_region: HirEagerExprRegion,
}

impl HasHirDecl for TraitPath {
    type HirDecl = TraitHirDecl;

    fn hir_decl(self, db: &dyn HirDeclDb) -> Option<Self::HirDecl> {
        todo!()
    }
}
