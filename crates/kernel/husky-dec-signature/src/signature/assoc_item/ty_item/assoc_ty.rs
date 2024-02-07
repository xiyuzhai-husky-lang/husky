use crate::*;

#[salsa::interned(db = DecSignatureDb, jar = DecSignatureJar)]
pub struct TypeAssocTypeDecTemplate {
    pub path: TypeItemPath,
    #[return_ref]
    pub template_parameters: DeclarativeTemplateParameterTemplates,
}

impl TypeAssocTypeDecTemplate {
    pub(super) fn from_decl(
        db: &::salsa::Db,
        path: TypeItemPath,
        decl: TypeAssocTypeSynDecl,
    ) -> DecSignatureResult<TypeAssocTypeDecTemplate> {
        let syn_expr_region = decl.syn_expr_region(db);
        let declarative_term_region = syn_expr_dec_term_region(db, syn_expr_region);
        let declarative_term_menu = db
            .declarative_term_menu(syn_expr_region.toolchain(db))
            .unwrap();
        let template_parameters = DeclarativeTemplateParameterTemplates::from_decl(
            decl.template_parameters(db),
            declarative_term_region,
            declarative_term_menu,
        );
        Ok(TypeAssocTypeDecTemplate::new(db, path, template_parameters))
    }
}
