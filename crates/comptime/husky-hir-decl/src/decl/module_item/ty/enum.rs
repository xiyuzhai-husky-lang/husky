use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar, constructor = new)]
pub struct EnumTypeHirDecl {
    pub path: TypePath,
    #[return_ref]
    pub template_parameters: HirTemplateParameters,
    pub hir_expr_region: HirEagerExprRegion,
}

impl EnumTypeHirDecl {
    pub(super) fn from_ethereal(
        path: TypePath,
        ethereal_signature_template: EnumTypeEtherealSignatureTemplate,
        db: &dyn HirDeclDb,
    ) -> Self {
        let mut builder = HirEagerExprBuilder::new(db);
        let template_parameters = HirTemplateParameters::from_ethereal(
            ethereal_signature_template.template_parameters(db),
            db,
        );
        let hir_expr_region = builder.finish();
        Self::new(db, path, template_parameters, hir_expr_region)
    }
}
