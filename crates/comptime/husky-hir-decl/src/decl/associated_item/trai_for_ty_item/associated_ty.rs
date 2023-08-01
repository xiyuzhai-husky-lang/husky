use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct TraitForTypeAssociatedTypeHirDecl {
    pub path: TraitForTypeItemPath,
    #[return_ref]
    pub template_parameters: HirTemplateParameters,
    pub associated_ty: HirType,
}

impl TraitForTypeAssociatedTypeHirDecl {
    pub(super) fn from_ethereal(
        path: TraitForTypeItemPath,
        template: TraitForTypeAssociatedTypeEtherealSignatureTemplate,
        db: &dyn HirDeclDb,
    ) -> Self {
        let template_parameters =
            HirTemplateParameters::from_ethereal(template.template_parameters(db), db);
        let associated_ty = HirType::from_ethereal(template.associated_ty(db), db);
        Self::new(db, path, template_parameters, associated_ty)
    }
}
