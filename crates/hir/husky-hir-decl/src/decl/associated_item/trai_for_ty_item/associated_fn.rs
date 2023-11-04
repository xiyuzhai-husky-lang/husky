use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct TraitForTypeAssociatedFnHirDecl {
    pub path: TraitForTypeItemPath,
    #[return_ref]
    pub template_parameters: HirTemplateParameters,
    #[return_ref]
    pub parenate_parameters: HirRitchieParameters,
    pub return_ty: HirType,
}

impl TraitForTypeAssociatedFnHirDecl {
    pub(super) fn from_ethereal(
        path: TraitForTypeItemPath,
        template: TraitForTypeAssociatedFnEtherealSignatureTemplate,
        db: &dyn HirDeclDb,
    ) -> Self {
        let template_parameters =
            HirTemplateParameters::from_ethereal(template.template_parameters(db), db);
        let parenate_parameters =
            HirRitchieParameters::from_ethereal(template.parenate_parameters(db), db);
        let return_ty = HirType::from_ethereal(template.return_ty(db), db);
        Self::new(
            db,
            path,
            template_parameters,
            parenate_parameters,
            return_ty,
        )
    }
}
