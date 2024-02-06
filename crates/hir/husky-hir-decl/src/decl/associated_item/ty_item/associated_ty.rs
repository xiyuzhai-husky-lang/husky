use super::*;
use husky_syn_decl::TypeAssocTypeSynDecl;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct TypeAssocTypeHirDecl {
    pub path: TypeItemPath,
    pub ty: HirType,
    pub hir_eager_expr_region: HirEagerExprRegion,
}

impl TypeAssocTypeHirDecl {
    pub(super) fn from_syn(
        path: TypeItemPath,
        syn_decl: TypeAssocTypeSynDecl,
        db: &::salsa::Db,
    ) -> Self {
        let builder = HirDeclBuilder::new(syn_decl.syn_expr_region(db), db);
        let ty = builder.hir_ty(syn_decl.ty_term_expr_idx(db)).unwrap();
        Self::new(db, path, ty, builder.finish().eager())
    }
}
