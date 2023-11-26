use super::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct ExternTypeDeclarativeSignatureTemplate {
    #[return_ref]
    pub template_parameters: DeclarativeTemplateParameterTemplates,
}

impl ExternTypeDeclarativeSignatureTemplate {
    pub(super) fn from_decl(
        db: &dyn DeclarativeSignatureDb,
        path: TypePath,
        decl: ExternTypeSynDecl,
    ) -> DeclarativeSignatureResult<Self> {
        let syn_expr_region = decl.syn_expr_region(db);
        let declarative_term_region = declarative_term_region(db, syn_expr_region);
        let declarative_term_menu = db
            .declarative_term_menu(syn_expr_region.toolchain(db))
            .unwrap();
        Ok(Self::new(
            db,
            DeclarativeTemplateParameterTemplates::from_decl(
                decl.template_parameters(db),
                &declarative_term_region,
                declarative_term_menu,
            ),
        ))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct ExternTypeDeclarativeSignature {}
