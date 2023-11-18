use super::*;
use husky_syn_decl::TypeAssociatedTypeSynDecl;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct TypeAssociatedTypeHirDecl {
    pub path: TypeItemPath,
    pub hir_eager_expr_region: HirEagerExprRegion,
}

impl TypeAssociatedTypeHirDecl {
    pub(super) fn from_syn(
        path: TypeItemPath,
        syn_decl: TypeAssociatedTypeSynDecl,
        db: &dyn HirDeclDb,
    ) -> Self {
        let builder = HirDeclBuilder::new(syn_decl.syn_expr_region(db), db);
        // let mut builder = HirEagerExprBuilder::new(db, syn_decl.expr_region(db));
        // // let return_ty = builder.hir_ty(ethereal_signature_template.return_ty(db), db);
        // let hir_expr_region = builder.finish();
        Self::new(db, path, builder.finish().eager())
    }
}
