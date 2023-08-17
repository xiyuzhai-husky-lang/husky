use super::*;
use husky_hir_ty::ritchie::HirRitchieParameter;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct TraitForTypeMethodFnHirDecl {
    pub path: TraitForTypeItemPath,
    #[return_ref]
    pub template_parameters: HirTemplateParameters,
    pub self_value_parameter: HirRitchieParameter,
    #[return_ref]
    pub parenate_parameters: HirParenateParameters,
    pub return_ty: HirType,
}

impl TraitForTypeMethodFnHirDecl {
    pub(super) fn from_ethereal(
        path: TraitForTypeItemPath,
        template: TraitForTypeMethodFnEtherealSignatureTemplate,
        db: &dyn HirDeclDb,
    ) -> Self {
        let template_parameters =
            HirTemplateParameters::from_ethereal(template.template_parameters(db), db);
        let self_value_parameter =
            HirRitchieParameter::from_ethereal_regular(template.self_value_parameter(db), db);
        let parenate_parameters =
            HirParenateParameters::from_ethereal(template.parenate_parameters(db), db);
        let return_ty = HirType::from_ethereal(template.return_ty(db), db);
        TraitForTypeMethodFnHirDecl::new(
            db,
            path,
            template_parameters,
            self_value_parameter,
            parenate_parameters,
            return_ty,
        )
    }
}
