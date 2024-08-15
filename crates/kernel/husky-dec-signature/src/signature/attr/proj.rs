use super::*;
use husky_syn_decl::decl::attr::proj::ProjAttrSynDecl;

#[salsa::interned]
pub struct ProjAttrDecTemplate {}

impl ProjAttrDecTemplate {
    pub(super) fn from_decl(decl: ProjAttrSynDecl, db: &::salsa::Db) -> DecSignatureResult<Self> {
        let syn_expr_region = decl.syn_expr_region(db);
        let dec_term_region = syn_expr_dec_term_region(db, syn_expr_region);
        Ok(Self::new(db))
    }
}
