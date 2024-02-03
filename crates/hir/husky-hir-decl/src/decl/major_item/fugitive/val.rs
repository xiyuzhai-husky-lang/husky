use super::*;
use husky_syn_decl::MajorValSynDecl;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct ValFugitiveHirDecl {
    pub path: MajorFugitivePath,
    pub return_ty: HirType,
    pub hir_eager_expr_region: HirEagerExprRegion,
}

impl ValFugitiveHirDecl {
    pub(super) fn from_syn(
        path: MajorFugitivePath,
        syn_decl: MajorValSynDecl,
        db: &::salsa::Db,
    ) -> Self {
        let builder = HirDeclBuilder::new(syn_decl.syn_expr_region(db), db);
        let return_ty = builder.return_ty_before_eq(syn_decl.return_ty(db));
        Self::new(db, path, return_ty, builder.finish().eager())
    }
}
