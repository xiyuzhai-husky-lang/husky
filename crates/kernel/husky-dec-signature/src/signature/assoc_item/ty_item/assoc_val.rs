use crate::*;
use husky_entity_path::path::assoc_item::ty_item::TypeItemPath;
use husky_syn_decl::decl::assoc_item::ty_item::assoc_val::TypeAssocValSynDecl;

#[salsa::interned]
pub struct TypeAssocValDecTemplate {
    pub path: TypeItemPath,
}

impl TypeAssocValDecTemplate {
    pub(super) fn from_decl(
        db: &::salsa::Db,
        path: TypeItemPath,
        decl: TypeAssocValSynDecl,
    ) -> DecSignatureResult<TypeAssocValDecTemplate> {
        let syn_expr_region = decl.syn_expr_region(db);
        let _dec_term_region = syn_expr_dec_term_region(db, syn_expr_region);
        let _dec_term_menu = db.dec_term_menu(syn_expr_region.toolchain(db)).unwrap();
        Ok(TypeAssocValDecTemplate::new(db, path))
    }
}
