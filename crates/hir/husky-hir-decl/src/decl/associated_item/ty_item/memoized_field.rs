use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct TypeMemoizedFieldHirDecl {
    pub path: TypeItemPath,
    pub return_ty: HirType,
    pub hir_eager_expr_region: HirEagerExprRegion,
}

impl TypeMemoizedFieldHirDecl {
    pub(super) fn from_syn(
        path: TypeItemPath,
        syn_decl: TypeMemoizedFieldEtherealSignatureTemplate,
        db: &dyn HirDeclDb,
    ) -> Self {
        let TypeItemSynDecl::MemoizedField(syn_decl) = path.syn_decl(db).expect("ok") else {
            unreachable!()
        };
        let builder = HirDeclBuilder::new(syn_decl.syn_expr_region(db), db);
        let return_ty = builder.return_ty(syn_decl.return_ty(db));
        Self::new(
            db,
            path,
            return_ty,
            hir_eager_expr_region(syn_decl.syn_expr_region(db), db),
        )
    }
}
