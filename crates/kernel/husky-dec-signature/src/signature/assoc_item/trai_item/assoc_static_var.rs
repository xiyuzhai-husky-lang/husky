use crate::*;
use husky_entity_path::path::assoc_item::trai_item::TraitItemPath;
use husky_syn_decl::decl::assoc_item::trai_item::assoc_static_var::TraitAssocStaticVarSynDecl;

#[salsa::interned]
pub struct TraitAssocStaticVarDecTemplate {
    pub path: TraitItemPath,
    pub return_ty: DecTerm,
    pub var_ty: DecTerm,
}

impl TraitAssocStaticVarDecTemplate {
    pub(super) fn from_decl(
        path: TraitItemPath,
        decl: TraitAssocStaticVarSynDecl,
        db: &::salsa::Db,
    ) -> DecSignatureResult<TraitAssocStaticVarDecTemplate> {
        let syn_expr_region = decl.syn_expr_region(db);
        let dec_term_region = syn_expr_dec_term_region(db, syn_expr_region);
        let _dec_term_menu = db.dec_term_menu(syn_expr_region.toolchain(db)).unwrap();
        let return_ty = dec_term_region.expr_term(decl.return_ty(db).syn_expr_idx())?;
        let var_ty = return_ty.var_ty(db);
        Ok(TraitAssocStaticVarDecTemplate::new(
            db, path, return_ty, var_ty,
        ))
    }
}
