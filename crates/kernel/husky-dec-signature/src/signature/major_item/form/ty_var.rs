use crate::*;
use husky_syn_decl::decl::major_item::form::ty_var::TypeVarSynDecl;

#[salsa::interned]
pub struct TypeVarDecTemplate {}

impl TypeVarDecTemplate {
    pub(super) fn from_decl(db: &::salsa::Db, decl: TypeVarSynDecl) -> DecSignatureResult<Self> {
        let syn_expr_region = decl.syn_expr_region(db);
        let dec_term_region = syn_expr_dec_term_region(db, syn_expr_region);
        let dec_term_menu = db.dec_term_menu(syn_expr_region.toolchain(db)).unwrap();
        Ok(TypeVarDecTemplate::new(db))
    }

    #[inline(always)]
    pub fn template_parameters(self, db: &::salsa::Db) -> &[DeclarativeTemplateParameter] {
        &[]
    }
}
