use super::*;
use husky_hir_ty::trai::HirTrait;
use husky_syn_decl::DeriveAttrSynDecl;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct DeriveAttrHirDecl {
    pub path: AttrItemPath,
    #[return_ref]
    pub trais: Vec<HirTrait>,
    pub hir_eager_expr_region: HirEagerExprRegion,
}

impl DeriveAttrHirDecl {
    pub(super) fn from_syn(
        path: AttrItemPath,
        syn_decl: DeriveAttrSynDecl,
        db: &::salsa::Db,
    ) -> Self {
        let builder = HirDeclBuilder::new(syn_decl.syn_expr_region(db), db);
        let trais = syn_decl
            .trais(db)
            .iter()
            .map(|syndicate| builder.hir_trai(syndicate.syn_expr_idx()))
            .collect();
        DeriveAttrHirDecl::new(db, path, trais, builder.finish().eager())
    }
}
