use super::*;
use husky_syn_decl::TraitForTypeAssociatedTypeSynDecl;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct TraitForTypeAssociatedTypeHirDecl {
    pub path: TraitForTypeItemPath,
    #[return_ref]
    pub template_parameters: HirTemplateParameters,
    pub associated_ty: HirType,
}

impl TraitForTypeAssociatedTypeHirDecl {
    pub(super) fn from_syn(
        path: TraitForTypeItemPath,
        syn_decl: TraitForTypeAssociatedTypeSynDecl,
        db: &dyn HirDeclDb,
    ) -> Self {
        let builder = HirDeclBuilder::new(syn_decl.syn_expr_region(db), db);
        let template_parameters =
            HirTemplateParameters::from_syn(syn_decl.template_parameters(db), db);
        let associated_ty = builder.hir_ty(syn_decl.ty_term_expr_idx(db));
        Self::new(db, path, template_parameters, associated_ty)
    }
}
