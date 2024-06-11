use crate::*;
use husky_entity_path::path::assoc_item::trai_item::TraitItemPath;
use husky_syn_decl::decl::assoc_item::trai_item::assoc_static_var::TraitAssocStaticVarSynDecl;

#[salsa::interned]
pub struct TraitAssocStaticVarDecTemplate {
    pub path: TraitItemPath,
}

impl TraitAssocStaticVarDecTemplate {
    pub(super) fn from_decl(
        path: TraitItemPath,
        decl: TraitAssocStaticVarSynDecl,
        db: &::salsa::Db,
    ) -> DecSignatureResult<TraitAssocStaticVarDecTemplate> {
        let syn_expr_region = decl.syn_expr_region(db);
        let _declarative_term_region = syn_expr_dec_term_region(db, syn_expr_region);
        let _declarative_term_menu = db.dec_term_menu(syn_expr_region.toolchain(db)).unwrap();
        Ok(TraitAssocStaticVarDecTemplate::new(db, path))
    }
}
