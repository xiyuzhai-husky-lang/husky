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
    pub(super) fn from_syn(
        path: TraitForTypeItemPath,
        template: TraitForTypeAssociatedValEtherealSignatureTemplate,
        db: &dyn HirDeclDb,
    ) -> Self {
        let template_parameters =
            HirTemplateParameters::from_syn(template.template_parameters(db), db);
        todo!()
        // let self_value_parameter =
        //     HirRitchieParameter::from_syn_regular(template.self_value_parameter(db), db);
        // let parenate_parameters =
        //     HirParenateParameters::from_syn(template.parenate_parameters(db), db);
        // let return_ty = HirType::from_syn(template.return_ty(db), db);
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
