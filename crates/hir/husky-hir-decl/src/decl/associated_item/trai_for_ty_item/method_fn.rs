use super::*;

use husky_syn_decl::TraitForTypeMethodFnSynDecl;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct TraitForTypeMethodFnHirDecl {
    pub path: TraitForTypeItemPath,
    #[return_ref]
    pub template_parameters: HirTemplateParameters,
    pub self_value_parameter: HirEagerSelfValueParameter,
    #[return_ref]
    pub parenate_parameters: HirEagerParenateParameters,
    pub return_ty: HirType,
    pub hir_eager_expr_region: HirEagerExprRegion,
}

impl TraitForTypeMethodFnHirDecl {
    pub(super) fn from_syn(
        path: TraitForTypeItemPath,
        syn_decl: TraitForTypeMethodFnSynDecl,
        db: &::salsa::Db,
    ) -> Self {
        let builder = HirDeclBuilder::new(syn_decl.syn_expr_region(db), db);
        let template_parameters =
            HirTemplateParameters::from_syn(syn_decl.template_parameters(db), &builder);
        let self_value_parameter =
            HirEagerSelfValueParameter::from_syn(syn_decl.self_value_parameter(db), db);
        let parenate_parameters =
            HirEagerParenateParameters::from_syn(syn_decl.parenate_parameters(db), &builder);
        let return_ty = syn_decl
            .return_ty(db)
            .map(|syndicate| builder.hir_ty(syndicate.syn_expr_idx()))
            .unwrap_or(builder.hir_ty_menu().unit_ty().into());
        TraitForTypeMethodFnHirDecl::new(
            db,
            path,
            template_parameters,
            self_value_parameter,
            parenate_parameters,
            return_ty,
            builder.finish().eager(),
        )
    }
}
