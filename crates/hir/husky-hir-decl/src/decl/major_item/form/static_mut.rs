use super::*;
use husky_syn_decl::decl::major_item::form::static_mut::MajorStaticMutSynDecl;

#[salsa::interned]
pub struct MajorStaticMutHirDecl {
    pub path: MajorFormPath,
    pub return_ty: HirType,
    pub hir_eager_expr_region: HirEagerExprRegion,
}

impl MajorStaticMutHirDecl {
    pub(super) fn from_syn(
        path: MajorFormPath,
        syn_decl: MajorStaticMutSynDecl,
        db: &::salsa::Db,
    ) -> Self {
        let builder = HirDeclBuilder::new(syn_decl.syn_expr_region(db), db);
        let return_ty = builder.return_ty_before_eq(syn_decl.return_ty(db));
        Self::new(db, path, return_ty, builder.finish().eager())
    }
}
