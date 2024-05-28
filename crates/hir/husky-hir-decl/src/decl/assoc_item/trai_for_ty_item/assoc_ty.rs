use super::*;
use husky_syn_decl::decl::assoc_item::trai_for_ty_item::assoc_ty::TraitForTypeAssocTypeSynDecl;

#[salsa::interned]
pub struct TraitForTypeAssocTypeHirDecl {
    pub path: TraitForTypeItemPath,
    #[return_ref]
    pub template_parameters: HirTemplateParameters,
    pub ty: HirType,
    pub hir_eager_expr_region: HirEagerExprRegion,
}

impl TraitForTypeAssocTypeHirDecl {
    pub(super) fn from_syn(
        path: TraitForTypeItemPath,
        syn_decl: TraitForTypeAssocTypeSynDecl,
        db: &::salsa::Db,
    ) -> Self {
        let builder = HirDeclBuilder::new(syn_decl.syn_expr_region(db), db);
        let template_parameters =
            HirTemplateParameters::from_syn(syn_decl.template_parameters(db), &builder);
        let ty = builder.hir_ty(syn_decl.ty_term_expr_idx(db)).unwrap();
        Self::new(db, path, template_parameters, ty, builder.finish().eager())
    }
}
