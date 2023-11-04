use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct TypeAliasFugitiveHirDecl {
    pub path: FugitivePath,
    #[return_ref]
    pub template_parameters: HirTemplateParameters,
}

impl TypeAliasFugitiveHirDecl {
    pub(super) fn from_ethereal(
        path: FugitivePath,
        ethereal_signature_template: TypeAliasFugitiveEtherealSignatureTemplate,
        db: &dyn HirDeclDb,
    ) -> Self {
        let template_parameters = HirTemplateParameters::from_ethereal(
            ethereal_signature_template.template_parameters(db),
            db,
        );
        Self::new(db, path, template_parameters)
    }
}
