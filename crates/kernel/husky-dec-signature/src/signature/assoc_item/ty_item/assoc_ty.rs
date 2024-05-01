use crate::*;

#[salsa::interned]
pub struct TypeAssocTypeDecTemplate {
    pub path: TypeItemPath,
    #[return_ref]
    pub template_parameters: DecTemplateParameters,
}

impl TypeAssocTypeDecTemplate {
    pub(super) fn from_decl(
        db: &::salsa::Db,
        path: TypeItemPath,
        decl: TypeAssocTypeSynDecl,
    ) -> DecSignatureResult<TypeAssocTypeDecTemplate> {
        let syn_expr_region = decl.syn_expr_region(db);
        let dec_term_region = syn_expr_dec_term_region(db, syn_expr_region);
        let dec_term_menu = db.dec_term_menu(syn_expr_region.toolchain(db)).unwrap();
        let template_parameters = DecTemplateParameters::from_decl(
            decl.template_parameters(db),
            dec_term_region,
            dec_term_menu,
        );
        Ok(TypeAssocTypeDecTemplate::new(db, path, template_parameters))
    }
}
