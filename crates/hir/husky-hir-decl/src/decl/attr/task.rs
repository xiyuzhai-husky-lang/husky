use super::*;
use husky_syn_decl::decl::task::TaskAttrSynDecl;

#[salsa::interned]
pub struct TaskAttrHirDecl {
    pub path: AttrItemPath,
}

impl TaskAttrHirDecl {
    pub(super) fn from_syn(
        path: AttrItemPath,
        syn_decl: TaskAttrSynDecl,
        db: &::salsa::Db,
    ) -> Self {
        let builder = HirDeclBuilder::new(syn_decl.syn_expr_region(db), db);
        Self::new(db, path)
    }
}
