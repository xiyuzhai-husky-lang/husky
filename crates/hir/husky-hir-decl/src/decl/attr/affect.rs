use super::*;
use husky_syn_decl::decl::attr::affect::AffectAttrSynDecl;

#[salsa::interned]
pub struct AffectAttrHirDecl {
    pub path: AttrItemPath,
}

pub struct HirAffectArgument {
    static_site: HirStaticSite,
}

pub enum HirStaticSite {}

impl AffectAttrHirDecl {
    pub(super) fn from_syn(
        path: AttrItemPath,
        syn_decl: AffectAttrSynDecl,
        db: &::salsa::Db,
    ) -> Self {
        let builder = HirDeclBuilder::new(syn_decl.syn_expr_region(db), db);
        Self::new(db, path)
    }
}
