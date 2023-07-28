use super::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct EnumDeclarativeSignatureTemplate {
    #[return_ref]
    pub template_parameters: DeclarativeTemplateParameterTemplates,
    pub self_ty: DeclarativeTerm,
}

impl EnumDeclarativeSignatureTemplate {
    pub fn from_decl(
        db: &dyn DeclarativeSignatureDb,
        path: TypePath,
        decl: EnumTypeSynDecl,
    ) -> DeclarativeSignatureResult<Self> {
        let syn_expr_region = decl.syn_expr_region(db);
        let declarative_term_region = declarative_term_region(db, syn_expr_region);
        let declarative_term_menu = db
            .declarative_term_menu(syn_expr_region.toolchain(db))
            .unwrap();
        let template_parameters = DeclarativeTemplateParameterTemplates::from_decl(
            decl.template_parameters(db),
            &declarative_term_region,
            declarative_term_menu,
        );
        let self_ty = construct_self_ty(db, path, &template_parameters);
        Ok(Self::new(db, template_parameters, self_ty))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = DeclarativeSignatureDb)]
pub struct EnumTypeDeclarativeSignature {}
