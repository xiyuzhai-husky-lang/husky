use super::*;
use husky_syn_decl::decl::test::TestAttrSynDecl;

#[salsa::interned]
pub struct TestAttrHirDecl {
    pub path: AttrItemPath,
}

impl TestAttrHirDecl {
    pub(super) fn from_syn(
        path: AttrItemPath,
        syn_decl: TestAttrSynDecl,
        db: &::salsa::Db,
    ) -> Self {
        let builder = HirDeclBuilder::new(syn_decl.syn_expr_region(db), db);
        Self::new(db, path)
    }
}
