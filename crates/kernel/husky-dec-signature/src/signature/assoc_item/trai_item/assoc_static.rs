use crate::*;
use husky_syn_decl::decl::assoc_item::trai_item::assoc_static::TraitAssocStaticSynDecl;

#[salsa::interned]
pub struct TraitAssocStaticDecTemplate {}

impl TraitAssocStaticDecTemplate {
    pub(super) fn from_decl(
        db: &::salsa::Db,
        decl: TraitAssocStaticSynDecl,
    ) -> DecSignatureResult<TraitAssocStaticDecTemplate> {
        let syn_expr_region = decl.syn_expr_region(db);
        let _declarative_term_region = syn_expr_dec_term_region(db, syn_expr_region);
        let _declarative_term_menu = db.dec_term_menu(syn_expr_region.toolchain(db)).unwrap();
        Ok(TraitAssocStaticDecTemplate::new(db))
    }
}
