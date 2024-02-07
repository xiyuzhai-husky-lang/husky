use crate::*;

#[salsa::interned(db = DecSignatureDb, jar = DecSignatureJar)]
pub struct TraitAssocTypeDecTemplate {}

impl TraitAssocTypeDecTemplate {
    pub(super) fn from_decl(
        db: &::salsa::Db,
        decl: TraitAssocTypeSynDecl,
    ) -> DecSignatureResult<TraitAssocTypeDecTemplate> {
        let syn_expr_region = decl.syn_expr_region(db);
        let _declarative_term_region = syn_expr_dec_term_region(db, syn_expr_region);
        let _declarative_term_menu = db
            .declarative_term_menu(syn_expr_region.toolchain(db))
            .unwrap();
        Ok(TraitAssocTypeDecTemplate::new(db))
    }
}
