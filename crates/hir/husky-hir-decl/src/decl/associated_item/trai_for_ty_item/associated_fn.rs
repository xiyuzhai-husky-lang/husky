use super::*;
use husky_syn_decl::TraitForTypeAssociatedFnSynDecl;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct TraitForTypeAssociatedFnHirDecl {
    pub path: TraitForTypeItemPath,
    #[return_ref]
    pub template_parameters: HirTemplateParameters,
    #[return_ref]
    pub parenate_parameters: HirParenateParameters,
    pub return_ty: HirType,
}

impl TraitForTypeAssociatedFnHirDecl {
    pub(super) fn from_syn(
        path: TraitForTypeItemPath,
        syn_decl: TraitForTypeAssociatedFnSynDecl,
        db: &dyn HirDeclDb,
    ) -> Self {
        let template_parameters =
            HirTemplateParameters::from_syn(syn_decl.template_parameters(db), db);
        let parenate_parameters =
            HirParenateParameters::from_syn(syn_decl.parenate_parameters(db), db);
        let return_ty = HirType::from_syn(syn_decl.return_ty(db), db);
        Self::new(
            db,
            path,
            template_parameters,
            parenate_parameters,
            return_ty,
        )
    }
}
