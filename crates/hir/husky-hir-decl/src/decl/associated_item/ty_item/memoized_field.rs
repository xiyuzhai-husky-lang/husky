use super::*;
use husky_syn_decl::TypeMemoizedFieldSynDecl;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct TypeMemoizedFieldHirDecl {
    pub path: TypeItemPath,
    pub return_ty: HirType,
    pub hir_eager_expr_region: HirEagerExprRegion,
}

impl TypeMemoizedFieldHirDecl {
    pub(super) fn from_syn(
        path: TypeItemPath,
        syn_decl: TypeMemoizedFieldSynDecl,
        db: &dyn HirDeclDb,
    ) -> Self {
        let builder = HirDeclBuilder::new(syn_decl.syn_expr_region(db), db);
        let return_ty = builder.return_ty_before_eq(syn_decl.return_ty(db));
        Self::new(
            db,
            path,
            return_ty,
            hir_eager_expr_region(syn_decl.syn_expr_region(db), db),
        )
    }
}
