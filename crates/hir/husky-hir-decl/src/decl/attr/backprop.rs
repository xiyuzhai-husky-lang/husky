use super::*;
use husky_syn_decl::decl::attr::backprop::BackpropAttrSynDecl;

#[salsa::interned]
pub struct BackpropAttrHirDecl {
    pub path: AttrItemPath,
}

impl BackpropAttrHirDecl {
    pub(super) fn from_syn(
        path: AttrItemPath,
        syn_decl: BackpropAttrSynDecl,
        db: &::salsa::Db,
    ) -> Self {
        let builder = HirDeclBuilder::new(syn_decl.syn_expr_region(db), db);
        Self::new(db, path)
    }
}
