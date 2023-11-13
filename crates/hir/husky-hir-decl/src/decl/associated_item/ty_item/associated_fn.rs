use husky_syn_decl::{HasSynDecl, TypeAssociatedFnSynDecl, TypeItemSynDecl};

use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct TypeAssociatedFnHirDecl {
    pub path: TypeItemPath,
    #[return_ref]
    pub template_parameters: HirTemplateParameters,
    #[return_ref]
    pub parenate_parameters: HirEagerParenateParameters,
    pub return_ty: HirType,
    pub hir_expr_region: HirEagerExprRegion,
}

impl TypeAssociatedFnHirDecl {
    pub(super) fn from_syn(
        path: TypeItemPath,
        _syn_decl: TypeAssociatedFnSynDecl,
        db: &dyn HirDeclDb,
    ) -> Self {
        let TypeItemSynDecl::AssociatedFn(syn_decl) = path.syn_decl(db).expect("ok") else {
            unreachable!()
        };
        let builder = HirDeclBuilder::new(syn_decl.syn_expr_region(db), db);
        let template_parameters =
            HirTemplateParameters::from_syn(syn_decl.template_parameters(db), &builder);
        let parenate_parameters =
            HirEagerParenateParameters::from_syn(syn_decl.parenate_parameters(db), &builder);
        let return_ty = syn_decl
            .return_ty(db)
            .map(|syndicate| builder.hir_ty(syndicate.syn_expr_idx()))
            .unwrap_or(builder.hir_ty_menu().unit_ty().into());
        Self::new(
            db,
            path,
            template_parameters,
            parenate_parameters,
            return_ty,
            hir_eager_expr_region(syn_decl.syn_expr_region(db), db),
        )
    }
}
