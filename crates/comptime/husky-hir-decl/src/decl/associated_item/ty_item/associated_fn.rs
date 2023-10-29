use husky_syn_decl::{HasSynDecl, TypeItemSynDecl, TypeSynDecl};

use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct TypeAssociatedFnHirDecl {
    pub path: TypeItemPath,
    #[return_ref]
    pub template_parameters: HirTemplateParameters,
    #[return_ref]
    pub parenate_parameters: HirParenateParameters,
    pub return_ty: HirType,
    pub hir_expr_region: HirEagerExprRegion,
}

impl TypeAssociatedFnHirDecl {
    pub(super) fn from_ethereal(
        path: TypeItemPath,
        ethereal_signature_template: TypeAssociatedFnEtherealSignatureTemplate,
        db: &dyn HirDeclDb,
    ) -> (Self, HirEagerExprSourceMap) {
        let TypeItemSynDecl::AssociatedFn(syn_decl) = path.syn_decl(db).expect("ok") else {
            unreachable!()
        };
        let mut builder = HirEagerExprBuilder::new(db, syn_decl.syn_expr_region(db));
        let template_parameters = HirTemplateParameters::from_ethereal(
            ethereal_signature_template.template_parameters(db),
            db,
        );
        let parenate_parameters = HirParenateParameters::from_ethereal(
            ethereal_signature_template.parenate_parameters(db),
            db,
        );
        let return_ty = HirType::from_ethereal(ethereal_signature_template.return_ty(db), db);
        let (hir_expr_region, hir_eager_expr_source_map) = builder.finish();
        (
            Self::new(
                db,
                path,
                template_parameters,
                parenate_parameters,
                return_ty,
                hir_expr_region,
            ),
            hir_eager_expr_source_map,
        )
    }
}
