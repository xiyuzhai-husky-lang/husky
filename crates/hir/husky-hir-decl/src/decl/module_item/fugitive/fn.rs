use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct FnFugitiveHirDecl {
    pub path: FugitivePath,
    #[return_ref]
    pub template_parameters: HirTemplateParameters,
}

impl FnFugitiveHirDecl {
    pub(super) fn from_ethereal(
        path: FugitivePath,
        ethereal_signature_template: FnFugitiveEtherealSignatureTemplate,
        db: &dyn HirDeclDb,
    ) -> Self {
        let template_parameters = HirTemplateParameters::from_ethereal(
            ethereal_signature_template.template_parameters(db),
            db,
        );
        Self::new(db, path, template_parameters)
    }
}
