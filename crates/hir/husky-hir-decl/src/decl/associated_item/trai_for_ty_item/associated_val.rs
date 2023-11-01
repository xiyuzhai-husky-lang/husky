use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct TraitForTypeAssociatedValHirDecl {
    pub path: TraitForTypeItemPath,
    #[return_ref]
    pub template_parameters: HirTemplateParameters,
    pub ty_term: EtherealTerm,
    pub hir_expr_region: HirEagerExprRegion,
}

impl TraitForTypeAssociatedValHirDecl {
    pub(super) fn from_ethereal(
        path: TraitForTypeItemPath,
        template: TraitForTypeAssociatedValEtherealSignatureTemplate,
        db: &dyn HirDeclDb,
    ) -> Self {
        let template_parameters =
            HirTemplateParameters::from_ethereal(template.template_parameters(db), db);
        todo!()
        // let self_value_parameter =
        //     HirRitchieParameter::from_ethereal_regular(template.self_value_parameter(db), db);
        // let parenate_parameters =
        //     HirParenateParameters::from_ethereal(template.parenate_parameters(db), db);
        // let return_ty = HirType::from_ethereal(template.return_ty(db), db);
        // TraitForTypeMethodFnHirDecl::new(
        //     db,
        //     path,
        //     template_parameters,
        //     self_value_parameter,
        //     parenate_parameters,
        //     return_ty,
        // )
    }
}
