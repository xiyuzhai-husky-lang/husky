use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct TraitHirDecl {
    pub path: TraitPath,
    #[return_ref]
    pub template_parameters: HirTemplateParameters,
    pub hir_eager_expr_region: HirEagerExprRegion,
}

impl HasHirDecl for TraitPath {
    type HirDecl = TraitHirDecl;

    fn hir_decl(self, db: &::salsa::Db) -> Option<Self::HirDecl> {
        Some(trai_hir_decl(db, self))
    }
}

#[salsa::tracked(jar = HirDeclJar)]
fn trai_hir_decl(db: &::salsa::Db, path: TraitPath) -> TraitHirDecl {
    let syn_decl = path.syn_decl(db).expect("ok");
    let builder = HirDeclBuilder::new(syn_decl.syn_expr_region(db), db);
    TraitHirDecl::new(
        db,
        path,
        HirTemplateParameters::from_syn(syn_decl.template_parameters(db), &builder),
        builder.finish().eager(),
    )
}
