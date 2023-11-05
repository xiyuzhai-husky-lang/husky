use husky_syn_decl::FunctionGnSynDecl;

use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct FunctionGnFugitiveHirDecl {
    pub path: FugitivePath,
    #[return_ref]
    pub template_parameters: HirTemplateParameters,
}

impl FunctionGnFugitiveHirDecl {
    pub(super) fn from_syn(
        path: FugitivePath,
        syn_decl: FunctionGnSynDecl,
        db: &dyn HirDeclDb,
    ) -> Self {
        let builder = HirDeclBuilder::new(syn_decl.syn_expr_region(db), db);
        let template_parameters =
            HirTemplateParameters::from_syn(syn_decl.template_parameters(db), &builder);
        let return_ty = syn_decl
            .return_ty(db)
            .map(|syndicate| builder.hir_ty(syndicate.syn_expr_idx()))
            .unwrap_or(builder.hir_ty_menu().unit_ty().into());
        Self::new(db, path, template_parameters)
    }
}
