use crate::*;

#[salsa::interned(db = DecSignatureDb, jar = DecSignatureJar)]
pub struct TraitForTypeAssociatedValDecTemplate {}

impl TraitForTypeAssociatedValDecTemplate {
    pub(super) fn from_decl(
        db: &::salsa::Db,
        decl: TraitForTypeAssociatedValSynDecl,
    ) -> DecSignatureResult<TraitForTypeAssociatedValDecTemplate> {
        let syn_expr_region = decl.syn_expr_region(db);
        let _declarative_term_region = syn_expr_dec_term_region(db, syn_expr_region);
        let _declarative_term_menu = db
            .declarative_term_menu(syn_expr_region.toolchain(db))
            .unwrap();
        Ok(TraitForTypeAssociatedValDecTemplate::new(db))
    }
}
