use super::*;
use husky_syn_decl::decl::attr::marker::MarkerAttrSynDecl;

#[salsa::interned]
pub struct MarkerAttrHirDecl {
    pub path: AttrItemPath,
}

impl MarkerAttrHirDecl {
    pub(super) fn from_syn(
        path: AttrItemPath,
        syn_decl: MarkerAttrSynDecl,
        db: &::salsa::Db,
    ) -> Self {
        let builder = HirDeclBuilder::new(syn_decl.syn_expr_region(db), db);
        Self::new(db, path)
    }
}
