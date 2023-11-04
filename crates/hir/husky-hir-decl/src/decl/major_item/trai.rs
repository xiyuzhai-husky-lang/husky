use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct TraitHirDecl {
    pub path: TraitPath,
    #[return_ref]
    pub template_parameters: HirTemplateParameters,
}

impl HasHirDecl for TraitPath {
    type HirDecl = TraitHirDecl;

    fn hir_decl(self, db: &dyn HirDeclDb) -> Option<Self::HirDecl> {
        Some(trai_hir_decl(db, self))
    }
}

#[salsa::tracked(jar = HirDeclJar)]
fn trai_hir_decl(db: &dyn HirDeclDb, path: TraitPath) -> TraitHirDecl {
    let syn_decl = path.syn_decl(db).expect("ok");
    TraitHirDecl::new(
        db,
        path,
        HirTemplateParameters::from_syn(syn_decl.template_parameters(db), db),
    )
}
