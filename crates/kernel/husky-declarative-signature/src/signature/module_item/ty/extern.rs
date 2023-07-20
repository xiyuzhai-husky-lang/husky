use super::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct ExternDeclarativeSignatureTemplate {
    #[return_ref]
    pub generic_parameters: DeclarativeGenericParameterTemplates,
}

impl ExternDeclarativeSignatureTemplate {
    pub(super) fn from_decl(
        db: &dyn DeclarativeSignatureDb,
        path: TypePath,
        decl: ExternTypeDecl,
    ) -> DeclarativeSignatureResult<Self> {
        let expr_region = decl.expr_region(db);
        let declarative_term_region = declarative_term_region(db, expr_region);
        let declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
        Ok(Self::new(
            db,
            DeclarativeGenericParameterTemplates::from_decl(
                decl.generic_parameters(db),
                &declarative_term_region,
                declarative_term_menu,
            ),
        ))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclarativeSignatureDb)]
pub struct ExternTypeDeclarativeSignature {}
