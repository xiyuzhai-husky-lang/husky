use super::*;
use husky_syn_decl::decl::r#static::MajorStaticSynDecl;

#[salsa::interned]
pub struct MajorStaticHirDecl {
    pub path: MajorFormPath,
    pub return_ty: HirType,
    pub hir_eager_expr_region: HirEagerExprRegion,
}

impl MajorStaticHirDecl {
    pub(super) fn from_syn(
        path: MajorFormPath,
        syn_decl: MajorStaticSynDecl,
        db: &::salsa::Db,
    ) -> Self {
        let builder = HirDeclBuilder::new(syn_decl.syn_expr_region(db), db);
        let return_ty = builder.return_ty_before_eq(syn_decl.return_ty(db));
        Self::new(db, path, return_ty, builder.finish().eager())
    }
}
