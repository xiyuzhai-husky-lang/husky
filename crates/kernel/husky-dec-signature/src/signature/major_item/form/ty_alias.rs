use crate::*;

#[salsa::interned]
pub struct TypeAliasDecTemplate {
    #[return_ref]
    pub template_parameters: DecTemplateParameters,
    pub ty_term: DecTerm,
}

impl TypeAliasDecTemplate {
    pub(super) fn from_decl(db: &::salsa::Db, decl: TypeAliasSynDecl) -> DecSignatureResult<Self> {
        let syn_expr_region = decl.syn_expr_region(db);
        let dec_term_region = syn_expr_dec_term_region(db, syn_expr_region);
        let dec_term_menu = db.dec_term_menu(syn_expr_region.toolchain(db)).unwrap();
        let template_parameters = DecTemplateParameters::from_decl(
            decl.template_parameters(db),
            &dec_term_region,
            dec_term_menu,
        );
        let ty_term = match decl.ty_term(db) {
            Some(ty_term) => dec_term_region.expr_term(ty_term)?,
            None => todo!(),
        };
        Ok(TypeAliasDecTemplate::new(db, template_parameters, ty_term))
    }
}
