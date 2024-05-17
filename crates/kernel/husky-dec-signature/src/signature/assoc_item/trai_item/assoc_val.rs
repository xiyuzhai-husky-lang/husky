use crate::*;

#[salsa::interned]
pub struct TraitAssocValDecTemplate {}

impl TraitAssocValDecTemplate {
    pub(super) fn from_decl(
        db: &::salsa::Db,
        decl: TraitAssocValSynDecl,
    ) -> DecSignatureResult<TraitAssocValDecTemplate> {
        let syn_expr_region = decl.syn_expr_region(db);
        let _declarative_term_region = syn_expr_dec_term_region(db, syn_expr_region);
        let _declarative_term_menu = db.dec_term_menu(syn_expr_region.toolchain(db)).unwrap();
        Ok(TraitAssocValDecTemplate::new(db))
    }
}
