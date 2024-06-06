use crate::*;
use husky_syn_decl::decl::major_item::form::ty_var::TypeVarSynDecl;

#[salsa::interned]
pub struct TypeVarDecTemplate {
    pub default: Option<DecTerm>,
}

impl TypeVarDecTemplate {
    pub(super) fn from_decl(db: &::salsa::Db, decl: TypeVarSynDecl) -> DecSignatureResult<Self> {
        let syn_expr_region = decl.syn_expr_region(db);
        let dec_term_region = syn_expr_dec_term_region(db, syn_expr_region);
        let default = match decl.default(db) {
            Some(ty_term) => Some(dec_term_region.expr_term(ty_term)?),
            None => None,
        };
        Ok(TypeVarDecTemplate::new(db, default))
    }

    #[inline(always)]
    pub fn template_parameters(self, db: &::salsa::Db) -> &[DeclarativeTemplateParameter] {
        &[]
    }
}
