use super::*;
use husky_syn_decl::decl::effect::EffectAttrSynDecl;

#[salsa::interned]
pub struct EffectAttrHirDecl {
    pub path: AttrItemPath,
}

impl EffectAttrHirDecl {
    pub(super) fn from_syn(
        path: AttrItemPath,
        syn_decl: EffectAttrSynDecl,
        db: &::salsa::Db,
    ) -> Self {
        let builder = HirDeclBuilder::new(syn_decl.syn_expr_region(db), db);
        Self::new(db, path)
    }
}
