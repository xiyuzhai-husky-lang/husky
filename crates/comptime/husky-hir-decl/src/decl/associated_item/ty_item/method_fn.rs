use super::*;
use husky_hir_ty::ritchie::HirRitchieParameter;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct TypeMethodFnHirDecl {
    pub path: TypeItemPath,
    pub self_ty: HirType,
    #[return_ref]
    pub template_parameters: HirTemplateParameters,
    pub self_value_parameter: HirRitchieParameter,
    #[return_ref]
    pub parenate_parameters: HirParenateParameters,
    pub return_ty: HirType,
}

impl TypeMethodFnHirDecl {
    pub(super) fn from_ethereal(
        path: TypeItemPath,
        ethereal_signature_template: TypeMethodFnEtherealSignatureTemplate,
        db: &dyn HirDeclDb,
    ) -> Self {
        let self_ty = HirType::from_ethereal(ethereal_signature_template.self_ty(db), db);
        let template_parameters = HirTemplateParameters::from_ethereal(
            ethereal_signature_template.template_parameters(db),
            db,
        );
        let self_value_parameter = HirRitchieParameter::from_ethereal_regular(
            ethereal_signature_template.self_value_parameter(db),
            db,
        );
        let parenate_parameters = HirParenateParameters::from_ethereal(
            ethereal_signature_template.parenate_parameters(db),
            db,
        );
        let return_ty = HirType::from_ethereal(ethereal_signature_template.return_ty(db), db);
        TypeMethodFnHirDecl::new(
            db,
            path,
            self_ty,
            template_parameters,
            self_value_parameter,
            parenate_parameters,
            return_ty,
        )
    }
}
