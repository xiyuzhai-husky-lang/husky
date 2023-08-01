use super::*;
use husky_hir_eager_expr::builder::HirEagerExprBuilder;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar, constructor = new_inner)]
pub struct ExternTypeHirDecl {
    pub path: TypePath,
    #[return_ref]
    pub template_parameters: HirTemplateParameters,
    pub hir_expr_region: HirEagerExprRegion,
}

impl ExternTypeHirDecl {
    pub(super) fn new(
        path: TypePath,
        ethereal_signature_template: ExternTypeEtherealSignatureTemplate,
        db: &dyn HirDeclDb,
    ) -> Self {
        let mut builder = HirEagerExprBuilder::new(db);
        let template_parameters = HirTemplateParameters::from_ethereal(
            ethereal_signature_template.template_parameters(db),
            db,
        );
        Self::new_inner(db, path, template_parameters, builder.finish())
    }
}
