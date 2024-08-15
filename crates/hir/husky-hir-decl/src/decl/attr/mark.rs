use super::*;
use husky_syn_decl::decl::attr::mark::MarkerAttrSynDecl;

#[salsa::interned]
pub struct MarkAttrHirDecl {
    pub path: AttrItemPath,
}

impl MarkAttrHirDecl {
    pub(super) fn from_syn(
        path: AttrItemPath,
        syn_decl: MarkerAttrSynDecl,
        db: &::salsa::Db,
    ) -> Self {
        let builder = HirDeclBuilder::new(syn_decl.syn_expr_region(db), db);
        Self::new(db, path)
    }
}
