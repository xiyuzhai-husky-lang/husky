use super::*;
use husky_syn_decl::decl::attr::singleton::SingletonAttrSynDecl;

#[salsa::interned]
pub struct SingletonAttrDecTemplate {}

impl SingletonAttrDecTemplate {
    pub(super) fn from_decl(
        decl: SingletonAttrSynDecl,
        db: &::salsa::Db,
    ) -> DecSignatureResult<Self> {
        let syn_expr_region = decl.syn_expr_region(db);
        let dec_term_region = syn_expr_dec_term_region(db, syn_expr_region);
        Ok(Self::new(db))
    }
}
