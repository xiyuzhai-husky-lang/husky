use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct TraitHirDecl {
    pub path: TraitPath,
    #[return_ref]
    pub template_parameters: HirTemplateParameters,
}

impl HasHirDecl for TraitPath {
    type HirDecl = TraitHirDecl;

    fn hir_decl_with_source_map(
        self,
        db: &dyn HirDeclDb,
    ) -> Option<(Self::HirDecl, Self::HirExprSourceMap)> {
        Some(trai_hir_decl(db, self))
    }
}

#[salsa::tracked(jar = HirDeclJar)]
fn trai_hir_decl(db: &dyn HirDeclDb, path: TraitPath) -> TraitHirDecl {
    let ethereal_signature_template = path.ethereal_signature_template(db).expect("ok");
    TraitHirDecl::new(
        db,
        path,
        HirTemplateParameters::from_ethereal(
            ethereal_signature_template.template_parameters(db),
            db,
        ),
    )
}
