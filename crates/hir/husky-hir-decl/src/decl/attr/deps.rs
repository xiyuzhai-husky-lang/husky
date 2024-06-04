use super::*;
use husky_hir_ty::trai::HirTrait;
use husky_syn_decl::decl::attr::deps::DepsAttrSynDecl;

#[salsa::interned]
pub struct DepsAttrHirDecl {
    pub path: AttrItemPath,
    pub hir_eager_expr_region: HirEagerExprRegion,
}

impl DepsAttrHirDecl {
    pub(super) fn from_syn(
        path: AttrItemPath,
        syn_decl: DepsAttrSynDecl,
        db: &::salsa::Db,
    ) -> Self {
        let builder = HirDeclBuilder::new(syn_decl.syn_expr_region(db), db);
        DepsAttrHirDecl::new(db, path, builder.finish().eager())
    }
}
