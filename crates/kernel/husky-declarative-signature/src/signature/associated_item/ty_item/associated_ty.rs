use crate::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct TypeAssociatedTypeDeclarativeSignatureTemplate {
    pub path: TypeItemPath,
    #[return_ref]
    pub template_parameters: DeclarativeTemplateParameterTemplates,
}

impl TypeAssociatedTypeDeclarativeSignatureTemplate {
    pub(super) fn from_decl(
        db: &::salsa::Db,
        path: TypeItemPath,
        decl: TypeAssociatedTypeSynDecl,
    ) -> DeclarativeSignatureResult<TypeAssociatedTypeDeclarativeSignatureTemplate> {
        let syn_expr_region = decl.syn_expr_region(db);
        let declarative_term_region = declarative_term_region(db, syn_expr_region);
        let declarative_term_menu = db
            .declarative_term_menu(syn_expr_region.toolchain(db))
            .unwrap();
        let template_parameters = DeclarativeTemplateParameterTemplates::from_decl(
            decl.template_parameters(db),
            declarative_term_region,
            declarative_term_menu,
        );
        Ok(TypeAssociatedTypeDeclarativeSignatureTemplate::new(
            db,
            path,
            template_parameters,
        ))
    }
}
