use super::*;
use husky_syn_decl::TraitForTypeAssociatedValSynDecl;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct TraitForTypeAssociatedValHirDecl {
    pub path: TraitForTypeItemPath,
    pub return_ty: HirType,
    pub hir_eager_expr_region: HirEagerExprRegion,
}

impl TraitForTypeAssociatedValHirDecl {
    pub(super) fn from_syn(
        _path: TraitForTypeItemPath,
        syn_decl: TraitForTypeAssociatedValSynDecl,
        db: &::salsa::Db,
    ) -> Self {
        let builder = HirDeclBuilder::new(syn_decl.syn_expr_region(db), db);
        let _template_parameters =
            HirTemplateParameters::from_syn(syn_decl.template_parameters(db), &builder);
        todo!()
        // let self_value_parameter =
        //     HirRitchieParameter::from_syn_regular(template.self_value_parameter(db), db);
        // let parenate_parameters =
        //     HirParenateParameters::from_syn(template.parenate_parameters(db), db);
        // let return_ty = builder.hir_ty(template.return_ty(db), db);
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
